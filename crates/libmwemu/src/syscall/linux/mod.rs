use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::syscall::linux::constants::errors::{EFAULT, EINVAL, ENOSYS};

pub mod constants;
pub mod files;
pub(crate) mod io;
pub(crate) mod mappings;
pub(crate) mod process;
pub mod syscall32;
pub mod syscall64;
pub mod syscall_aarch64;
mod time;
pub(crate) mod tls32;

pub(super) fn unsupported(emu: &mut Emu, name: &str) {
    let call = LinuxSyscall;
    log::trace!("{} syscall {name}: unsupported", emu.pos);
    call.set_return(emu, ENOSYS);
}

pub(crate) fn current_tid(emu: &Emu) -> u64 {
    if emu.current_thread_id == 0 {
        emu.processes.pid
    } else {
        emu.threads[emu.current_thread_id].id
    }
}

fn getpid(emu: &mut Emu) {
    let call = LinuxSyscall;
    log::trace!("{} syscall getpid: {}", emu.pos, emu.processes.pid);
    call.set_return(emu, emu.processes.pid);
}

fn getppid(emu: &mut Emu) {
    let call = LinuxSyscall;
    log::trace!("{} syscall getppid: {}", emu.pos, emu.processes.ppid);
    call.set_return(emu, emu.processes.ppid);
}

fn gettid(emu: &mut Emu) {
    let call = LinuxSyscall;
    let tid = current_tid(emu);
    log::trace!("{} syscall gettid: {tid}", emu.pos);
    call.set_return(emu, tid);
}

fn set_tid_address(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let address = call.args_for(emu, arch)[0];
    let tid = current_tid(emu);
    emu.threads[emu.current_thread_id].clear_child_tid = address;
    log::trace!("{} syscall set_tid_address({address:#x}): {tid}", emu.pos);
    call.set_return(emu, tid);
}

pub(super) fn uname(emu: &mut Emu, arch: Arch) {
    const FIELD_SIZE: usize = 65;
    let call = LinuxSyscall;
    let [address, ..] = call.args_for(emu, arch);
    let fields = match emu.cfg.arch {
        Arch::Aarch64 => ["Linux", "mwemu", "5.15.0", "#1 SMP", "aarch64", "(none)"],
        arch => {
            let machine = if arch == Arch::X86 { "i686" } else { "x86_64" };
            [
                "Linux",
                "saturn",
                "5.10.0-23-amd64",
                "#1 SMP Debian 5.10.179-1 (2023-05-12)",
                machine,
                "(none)",
            ]
        }
    };
    let mut data = [0; FIELD_SIZE * 6];
    for (field, slot) in fields.iter().zip(data.chunks_exact_mut(FIELD_SIZE)) {
        slot[..field.len()].copy_from_slice(field.as_bytes());
    }
    let ok = address != 0
        && emu.maps.validate_write_range(address, data.len() as u64)
        && emu.maps.write_bytes(address, &data);
    log::trace!("Linux uname({address:#x})");
    call.set_return(emu, if ok { 0 } else { EFAULT });
}

pub(super) fn getrandom(emu: &mut Emu, arch: Arch) {
    const GRND_NONBLOCK: u32 = 1;
    const GRND_RANDOM: u32 = 2;
    const GRND_INSECURE: u32 = 4;
    let call = LinuxSyscall;
    let [address, size, flags, ..] = call.args_for(emu, arch);
    let flags = flags as u32;
    let incompatible = GRND_RANDOM | GRND_INSECURE;
    let result =
        if flags & !(GRND_NONBLOCK | incompatible) != 0 || flags & incompatible == incompatible {
            EINVAL
        } else if emu.fill_random(address, size) {
            size
        } else {
            EFAULT
        };
    log::trace!("Linux getrandom({address:#x}, {size}, {flags:#x})");
    call.set_return(emu, result);
}

pub(crate) const GUEST_ID: u32 = 1000;

fn get_identity(emu: &mut Emu) {
    let call = LinuxSyscall;
    call.set_return(emu, GUEST_ID.into());
}

fn get_res_ids(emu: &mut Emu, arch: Arch, legacy: bool) {
    let call = LinuxSyscall;
    let [real, effective, saved, ..] = call.args_for(emu, arch);
    let bytes = GUEST_ID.to_le_bytes();
    let id = if legacy {
        &bytes[..size_of::<u16>()]
    } else {
        &bytes
    };
    let ok = [real, effective, saved]
        .into_iter()
        .all(|address| address != 0 && emu.maps.write_bytes(address, id));
    call.set_return(emu, if ok { 0 } else { EFAULT });
}

fn get_groups(emu: &mut Emu, arch: Arch, legacy: bool) {
    let call = LinuxSyscall;
    let [size, out, ..] = call.args_for(emu, arch);
    let bytes = GUEST_ID.to_le_bytes();
    let id = if legacy {
        &bytes[..size_of::<u16>()]
    } else {
        &bytes
    };
    let result = match size as i32 {
        n if n < 0 => EINVAL,
        0 => 1,
        _ if out != 0 && emu.maps.write_bytes(out, id) => 1,
        _ => EFAULT,
    };
    call.set_return(emu, result);
}
