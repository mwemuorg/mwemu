use std::ffi::CStr;
use std::time::Duration;

use goblin::elf::{Elf, program_header::PT_INTERP};
use iced_x86::{Decoder, DecoderOptions};

use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::syscall::linux::constants::errors::{EAGAIN, ECHILD, EFAULT, EINVAL, ENOEXEC, ENOSYS};
use crate::syscall::linux::files::MAX_PATH;
use crate::syscall::linux::syscall64::vfork_parent;
use crate::threading::process::exit_status;

const MAX_SAVED_BYTES: usize = 128 << 20;
const MAX_INSTRUCTION_BYTES: usize = 15;

fn can_save_process(emu: &Emu) -> bool {
    MAX_SAVED_BYTES
        .checked_sub(emu.maps.size())
        .is_some_and(|left| emu.processes.can_fork_within(left))
}

pub(super) fn fork(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let pc = emu.pc();
    if emu.cfg.arch != arch {
        call.set_return(emu, ENOSYS);
        return;
    }
    if !can_save_process(emu) {
        call.set_return(emu, EAGAIN);
        return;
    }
    let Some(next_pc) = next_instruction(emu) else {
        call.set_return(emu, EFAULT);
        return;
    };
    // Save the parent's syscall continuation, including any instruction prefixes.
    emu.set_pc(next_pc);
    call.set_return(emu, emu.processes.next_child_pid());
    emu.fork_begin();
    emu.set_pc(pc);
    call.set_return(emu, 0);
}

pub(super) fn exit(emu: &mut Emu) {
    let call = LinuxSyscall;
    let [code, ..] = call.args_for(emu, Arch::X86);
    log::trace!("linux i386 syscall exit({code})");
    if emu.process_exit(exit_status(code)).is_some() {
        emu.force_reload = true;
    }
}

fn next_instruction(emu: &Emu) -> Option<u64> {
    let pc = emu.pc();
    let bytes = emu
        .maps
        .get_mem_by_addr(pc)?
        .read_code(pc, MAX_INSTRUCTION_BYTES);
    let bitness = if emu.cfg.is_x64() { 64 } else { 32 };
    let ins = Decoder::with_ip(bitness, bytes.as_ref(), pc, DecoderOptions::NONE).decode();
    (!ins.is_invalid()).then(|| ins.next_ip())
}

pub(super) fn nanosleep(emu: &mut Emu) {
    let call = LinuxSyscall;
    let [request, ..] = call.args_for(emu, Arch::X86);
    let Some((sec, nsec)) = emu
        .maps
        .read_qword(request)
        .map(|ts| (ts as i32, (ts >> 32) as i32))
    else {
        call.set_return(emu, EFAULT);
        return;
    };
    if sec < 0 || !(0..1_000_000_000).contains(&nsec) {
        call.set_return(emu, EINVAL);
        return;
    }
    let duration = Duration::new(sec as u64, nsec as u32);
    if !duration.is_zero() && emu.processes.in_child() {
        let Some(next_pc) = next_instruction(emu) else {
            call.set_return(emu, EFAULT);
            return;
        };
        emu.set_pc(next_pc);
    }
    call.set_return(emu, 0);
    if !duration.is_zero() {
        emu.sleep_linux_process(duration);
    }
}

pub(super) fn wait_child(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [pid, address, options, fourth, ..] = call.args_for(emu, arch);
    // i386 waitpid takes three arguments. amd64 wait4 also accepts a struct rusage.
    let usage = if arch == Arch::X86_64 { fourth } else { 0 };
    const RUSAGE64_SIZE: usize = 144;
    let pid = pid as i32;
    const WNOHANG: u64 = 1;
    let result = if options & !WNOHANG != 0 || pid == 0 || pid < -1 {
        EINVAL
    } else if (address != 0 && !emu.maps.validate_write_range(address, 4))
        || (usage != 0 && !emu.maps.validate_write_range(usage, RUSAGE64_SIZE as u64))
    {
        EFAULT
    } else if let Some((pid, status)) = emu.reap_child(pid as i64) {
        if address != 0 {
            emu.maps.write_dword(address, status as u32);
        }
        if usage != 0 {
            emu.maps.write_bytes(usage, &[0; RUSAGE64_SIZE]);
        }
        log::trace!("linux wait returned pid={pid} status={status:#x}");
        pid
    } else if let Some(index) = emu.processes.sleeping_child(pid) {
        if options & WNOHANG == 0 {
            emu.resume_sleeping_process(index);
            return;
        }
        0
    } else {
        ECHILD
    };
    call.set_return(emu, result);
}

fn interpreter_path<'a>(elf: &Elf<'_>, bytes: &'a [u8]) -> Result<Option<&'a str>, u64> {
    let mut headers = elf.program_headers.iter().filter(|p| p.p_type == PT_INTERP);
    let Some(ph) = headers.next() else {
        return Ok(None);
    };
    if headers.next().is_some() || !(2..=MAX_PATH as u64).contains(&ph.p_filesz) {
        return Err(ENOEXEC);
    }
    let start = usize::try_from(ph.p_offset).map_err(|_| ENOEXEC)?;
    let end = start.checked_add(ph.p_filesz as usize).ok_or(ENOEXEC)?;
    let field = bytes.get(start..end).ok_or(ENOEXEC)?;
    if field.last() != Some(&0) {
        return Err(ENOEXEC);
    }
    // PT_INTERP can reserve more space than the current NUL-terminated path.
    let path = CStr::from_bytes_until_nul(field)
        .map_err(|_| ENOEXEC)?
        .to_str()
        .map_err(|_| ENOEXEC)?;
    Ok(Some(path))
}

/// Keep the running engine, its observation hooks and its cumulative limits across exec.
pub(super) fn exec_image(emu: &mut Emu) -> Result<(), u64> {
    let parent = vfork_parent(emu);
    if parent.is_some() && !can_save_process(emu) {
        return Err(EAGAIN);
    }
    let files = emu.linux_files.as_ref().ok_or(ENOEXEC)?;
    let (path, bytes, argv, envp) = files.exec_request().ok_or(ENOEXEC)?;
    let elf = Elf::parse(bytes).map_err(|_| ENOEXEC)?;
    let interpreter = interpreter_path(&elf, bytes)?
        .map(|name| files.executable_bytes(name))
        .transpose()?;
    let arch = if elf.is_64 { Arch::X86_64 } else { Arch::X86 };
    let mut next = Emu::new(arch);
    next.cfg.exe_name = argv.first().cloned().unwrap_or_default();
    next.set_linux_files(Vec::new(), Vec::new());
    if let Some(fs) = &mut next.linux_files {
        fs.args.extend(argv.iter().skip(1).cloned());
        fs.env.extend_from_slice(envp);
    }
    if elf.is_64 {
        next.load_guest_elf64(path, bytes, &elf, interpreter)?;
    } else {
        next.load_guest_elf32(path, bytes, &elf, interpreter)?;
    }
    emu.sync_linux_files();
    emu.enter_linux_exec(parent)?;
    let files = emu.linux_files.as_mut().ok_or(ENOEXEC)?;
    if let Some(fresh) = &next.linux_files {
        files.copy_process_auxv(fresh);
    }
    emu.cfg.arch = arch;
    emu.maps = next.maps;
    emu.threads = next.threads;
    emu.current_thread_id = 0;
    emu.heap_addr = next.heap_addr;
    emu.heap_arenas = next.heap_arenas;
    emu.elf32 = next.elf32;
    emu.elf64 = next.elf64;
    emu.ld_bootstrap = next.ld_bootstrap;
    emu.reset_active_instruction_cache();
    emu.force_reload = true;
    Ok(())
}
