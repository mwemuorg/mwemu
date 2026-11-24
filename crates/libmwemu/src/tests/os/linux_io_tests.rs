use crate::api::linux::libc;
use crate::api::unix;
use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{
    EAGAIN, EBADF, EFAULT, EINVAL, ENOENT, ENOEXEC, ENOSYS, ENOTDIR, EPIPE,
};
use crate::syscall::linux::constants::io::*;
use crate::syscall::linux::{syscall_aarch64, syscall32, syscall64};
use crate::tests::helpers;
use crate::{emu_aarch64, emu32, emu64};

const DATA: u64 = 0x80000;
const IOVEC: u64 = 0x90000;
const ARCHITECTURES: [Arch; 3] = [Arch::X86, Arch::X86_64, Arch::Aarch64];

#[derive(Clone, Copy, Debug)]
enum Operation {
    Read,
    Write,
    Writev,
    Fcntl,
    Pipe2,
    Eventfd2,
    OpenAt,
    GetDents64,
    SymlinkAt,
    AccessAt,
    UnlinkAt,
    ReadlinkAt,
    Exec,
    DupTo,
    Memfd,
    Seek,
    Close,
}

impl Operation {
    fn number(self, arch: Arch) -> u64 {
        use crate::syscall::linux::constants::syscalls::{
            NR32_GETDENTS64, NR32_SYMLINKAT, NR64_GETDENTS64, NR64_SYMLINKAT, NRA64_GETDENTS64,
            NRA64_SYMLINKAT,
        };
        let numbers = match self {
            Self::Read => [3, 0, 63],
            Self::Write => [4, 1, 64],
            Self::Writev => [146, 20, 66],
            Self::Fcntl => [55, 72, 25],
            Self::Pipe2 => [331, 293, 59],
            Self::Eventfd2 => [328, 290, 19],
            Self::GetDents64 => [NR32_GETDENTS64, NR64_GETDENTS64, NRA64_GETDENTS64],
            Self::SymlinkAt => [NR32_SYMLINKAT, NR64_SYMLINKAT, NRA64_SYMLINKAT],
            Self::OpenAt => [295, 257, 56],
            Self::AccessAt => [307, 269, 48],
            Self::UnlinkAt => [301, 263, 35],
            Self::ReadlinkAt => [305, 267, 78],
            Self::Exec => [11, 59, 221],
            Self::DupTo => [63, 33, 24],
            Self::Memfd => [356, 319, 279],
            Self::Seek => [19, 8, 62],
            Self::Close => [6, 3, 57],
        };
        numbers[match arch {
            Arch::X86 => 0,
            Arch::X86_64 => 1,
            Arch::Aarch64 => 2,
        }]
    }
}

fn environment(arch: Arch) -> Emu {
    helpers::setup();
    let mut emu = match arch {
        Arch::X86 => emu32(),
        Arch::X86_64 => emu64(),
        Arch::Aarch64 => emu_aarch64(),
    };
    emu.os = OperatingSystem::Linux;
    emu.maps
        .create_map("io data", DATA, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps
        .create_map("io vectors", IOVEC, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu
}

fn invoke(emu: &mut Emu, operation: Operation, args: [u64; 3]) -> i64 {
    let [a, b, c] = args;
    invoke_with(emu, operation, [a, b, c, 0])
}

fn invoke_with(emu: &mut Emu, operation: Operation, args: [u64; 4]) -> i64 {
    let number = operation.number(emu.cfg.arch);
    match emu.cfg.arch {
        Arch::X86 => {
            let regs = emu.regs_mut();
            regs.rax = number;
            [regs.rbx, regs.rcx, regs.rdx, regs.rsi] = args;
            syscall32::gateway(emu);
            emu.regs().get_eax() as i32 as i64
        }
        Arch::X86_64 => {
            let regs = emu.regs_mut();
            regs.rax = number;
            [regs.rdi, regs.rsi, regs.rdx, regs.r10] = args;
            syscall64::gateway(emu);
            emu.regs().rax as i64
        }
        Arch::Aarch64 => {
            let regs = emu.regs_aarch64_mut();
            regs.x[8] = number;
            regs.x[..args.len()].copy_from_slice(&args);
            syscall_aarch64::gateway(emu);
            emu.regs_aarch64().x[0] as i64
        }
    }
}

fn vectors(emu: &mut Emu, entries: &[(u64, u64)]) {
    let mut bytes = Vec::new();
    for &(address, length) in entries {
        for word in [address, length] {
            if emu.cfg.arch.is_64bits() {
                bytes.extend_from_slice(&word.to_le_bytes());
            } else {
                bytes.extend_from_slice(&(word as u32).to_le_bytes());
            }
        }
    }
    assert!(emu.maps.write_bytes(IOVEC, &bytes));
}

#[test]
fn i386_exec_starts_the_guest_interpreter_with_the_main_program_auxv() {
    use goblin::elf::header::{ET_DYN, ET_EXEC};
    for kind in [ET_EXEC, ET_DYN] {
        check_i386_interpreter_handoff(kind);
    }
}

fn check_i386_interpreter_handoff(kind: u16) {
    use crate::syscall::linux::constants::elf::*;
    use crate::syscall::linux::constants::syscalls::NR32_EXECVE;
    use byteorder::{LittleEndian as LE, WriteBytesExt};
    use goblin::elf::dynamic::{DT_NULL, DT_REL, DT_RELENT, DT_RELSZ};
    use goblin::elf::header::{EM_386, ET_DYN, ET_EXEC};
    use goblin::elf::program_header::{PF_R, PF_W, PF_X, PT_DYNAMIC, PT_INTERP, PT_LOAD};
    use goblin::elf::reloc::R_386_RELATIVE;
    use std::collections::BTreeMap;

    const MAIN_BASE: u32 = 0x0804_8000;
    const PIE_ALIGNMENT: u32 = 0x20000;
    const ENTRY_OFFSET: u32 = 0x100;
    const DYNAMIC_OFFSET: u32 = 0x140;
    const DYNAMIC_SIZE: u32 = 4 * 8;
    const REL_OFFSET: u32 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
    const REL_SIZE: u32 = 8;
    const REL_SLOT: u32 = REL_OFFSET + REL_SIZE;
    const ADDEND: u32 = 0x78;
    const PHOFF: u32 = 52;
    const PHENT: u16 = 32;
    const INTERP: &str = "/lib/test-interpreter";
    const MAIN_PATH: &str = "/tmp/program";
    const DIR: u64 = (-100i64) as u64;
    const CREATE_RDWR: u64 = 0o102;
    const CLOEXEC: u64 = 0o2000000;

    fn image(base: u32, kind: u16, has_interp: bool, code: &[u8]) -> Vec<u8> {
        const INTERP_OFFSET: u32 = 0xc0;
        let relocatable_main = has_interp && kind == ET_DYN;
        let size = if relocatable_main {
            REL_SLOT + 4
        } else {
            ENTRY_OFFSET + code.len() as u32
        };
        let mut bytes = vec![0x7f, b'E', b'L', b'F', 1, 1, 1];
        bytes.resize(16, 0);
        for word in [kind, EM_386] {
            bytes.write_u16::<LE>(word).unwrap();
        }
        for word in [1, base + ENTRY_OFFSET, PHOFF, 0, 0] {
            bytes.write_u32::<LE>(word).unwrap();
        }
        let count = 1 + u16::from(has_interp) + u16::from(relocatable_main);
        for word in [PHOFF as u16, PHENT, count, 0, 0, 0] {
            bytes.write_u16::<LE>(word).unwrap();
        }
        let align = if relocatable_main {
            PIE_ALIGNMENT
        } else {
            4096
        };
        let flags = PF_R | PF_X | if relocatable_main { PF_W } else { 0 };
        for word in [PT_LOAD, 0, base, base, size, size, flags, align] {
            bytes.write_u32::<LE>(word).unwrap();
        }
        if has_interp {
            let padded_path = [INTERP.as_bytes(), &[0, 0xff, 0]].concat();
            let len = padded_path.len() as u32;
            for word in [
                PT_INTERP,
                INTERP_OFFSET,
                base + INTERP_OFFSET,
                base + INTERP_OFFSET,
                len,
                len,
                PF_R,
                1,
            ] {
                bytes.write_u32::<LE>(word).unwrap();
            }
            if relocatable_main {
                for word in [
                    PT_DYNAMIC,
                    DYNAMIC_OFFSET,
                    DYNAMIC_OFFSET,
                    DYNAMIC_OFFSET,
                    DYNAMIC_SIZE,
                    DYNAMIC_SIZE,
                    PF_R,
                    4,
                ] {
                    bytes.write_u32::<LE>(word).unwrap();
                }
            }
            bytes.resize(INTERP_OFFSET as usize, 0);
            bytes.extend_from_slice(&padded_path);
        }
        bytes.resize(ENTRY_OFFSET as usize, 0);
        bytes.extend_from_slice(code);
        if relocatable_main {
            bytes.resize(DYNAMIC_OFFSET as usize, 0);
            for (tag, value) in [
                (DT_REL, REL_OFFSET),
                (DT_RELSZ, REL_SIZE),
                (DT_RELENT, REL_SIZE),
                (DT_NULL, 0),
            ] {
                bytes.write_u32::<LE>(tag as u32).unwrap();
                bytes.write_u32::<LE>(value).unwrap();
            }
            for word in [REL_SLOT, R_386_RELATIVE, ADDEND] {
                bytes.write_u32::<LE>(word).unwrap();
            }
        }
        bytes
    }

    let mut emu = environment(Arch::X86);
    emu.set_linux_files(b"parent".to_vec(), Vec::new());
    let link_base = if kind == ET_EXEC { MAIN_BASE } else { 0 };
    let main = image(link_base, kind, true, &[0x90]); // nop
    emu.maps.write_string(DATA, MAIN_PATH);
    let writer = invoke_with(&mut emu, Operation::OpenAt, [DIR, DATA, CREATE_RDWR, 0o700]);
    assert!(writer >= 3);
    emu.maps.write_bytes(IOVEC, &main);
    assert_eq!(
        invoke(
            &mut emu,
            Operation::Write,
            [writer as u64, IOVEC, main.len() as u64]
        ),
        main.len() as i64
    );
    assert_eq!(invoke(&mut emu, Operation::Close, [writer as u64, 0, 0]), 0);
    let image_fd = invoke_with(&mut emu, Operation::OpenAt, [DIR, DATA, CLOEXEC, 0]);
    assert!(image_fd >= 3);
    assert_eq!(invoke(&mut emu, Operation::UnlinkAt, [DIR, DATA, 0]), 0);
    let filename = format!("/proc/{}/fd/{image_fd}", emu.processes.pid);
    emu.maps.write_string(DATA, &filename);
    emu.maps.write_bytes(IOVEC, &[0; 16]);
    emu.maps.write_string(DATA + 64, "alternate-argv-zero");
    emu.maps.write_string(DATA + 128, "LANG=C");
    emu.maps.write_dword(IOVEC, (DATA + 64) as u32);
    emu.maps.write_dword(IOVEC + 8, (DATA + 128) as u32);
    let exec = |emu: &mut Emu| {
        let regs = emu.regs_mut();
        regs.rax = NR32_EXECVE;
        [regs.rbx, regs.rcx, regs.rdx] = [DATA, IOVEC, IOVEC + 8];
        syscall32::gateway(emu);
    };
    let before = emu.pc();
    exec(&mut emu);
    assert_eq!(emu.regs().get_eax() as u32, ENOENT as u32);
    assert_eq!(emu.pc(), before);
    assert_eq!(emu.maps.read_string(DATA), filename);
    let files = emu.linux_files.as_ref().unwrap();
    assert!(files.attempted_exec().is_none());
    assert_eq!(
        files.read_at(image_fd as u32, 0, main.len()),
        Ok(main.as_slice())
    );

    // The guest interpreter walks auxv after argc, argv and envp, then jumps to AT_ENTRY.
    let mut handoff = vec![
        0x8d,
        0x7c,
        0x24,
        0x14, // lea edi, [esp+20]
        0x83,
        0x3f,
        AT_ENTRY as u8, // cmp dword [edi], AT_ENTRY
        0x74,
        5, // je found
        0x83,
        0xc7,
        8, // add edi, 8
        0xeb,
        0xf6, // jmp compare
        0x8b,
        0x47,
        4, // found: mov eax, [edi+4]
    ];
    if kind == ET_DYN {
        // Apply R_386_RELATIVE to the linked slot before entering the PIE.
        handoff.extend([0x8d, 0x90]); // lea edx, [eax-ENTRY_OFFSET]
        handoff.extend(ENTRY_OFFSET.wrapping_neg().to_le_bytes());
        handoff.extend([0x01, 0x92]); // add dword [edx+REL_SLOT], edx
        handoff.extend(REL_SLOT.to_le_bytes());
    }
    handoff.extend([0xff, 0xe0]); // jmp eax
    emu.linux_files
        .as_mut()
        .unwrap()
        .mount_executable(INTERP, &image(0, ET_DYN, false, &handoff))
        .unwrap();
    exec(&mut emu);
    let sp = emu.sp();
    let stack = |index| emu.maps.read_dword(sp + index * 4u64).unwrap() as u64;
    assert_eq!(stack(0), 1);
    assert_eq!(emu.maps.read_string(stack(1)), "alternate-argv-zero");
    assert_eq!(stack(2), 0);
    assert_eq!(emu.maps.read_string(stack(3)), "LANG=C");
    assert_eq!(stack(4), 0);
    let auxv: BTreeMap<_, _> = (5..)
        .step_by(2)
        .map(|i| (stack(i), stack(i + 1)))
        .take_while(|&(tag, _)| tag != 0)
        .collect();
    let main_base = auxv[&AT_PHDR] - PHOFF as u64;
    if kind == ET_EXEC {
        assert_eq!(main_base, MAIN_BASE as u64);
    } else {
        assert_ne!(main_base, 0);
        assert!(main_base.is_multiple_of(PIE_ALIGNMENT as u64));
    }
    assert_ne!(main_base, auxv[&AT_BASE]);
    assert_eq!(emu.maps.read_bytes(main_base, 4).as_ref(), b"\x7fELF");
    let main_entry = main_base + ENTRY_OFFSET as u64;
    assert_eq!(auxv[&AT_PHENT], PHENT as u64);
    assert_eq!(auxv[&AT_PHNUM], if kind == ET_DYN { 3 } else { 2 });
    assert_eq!(auxv[&AT_ENTRY], main_entry);
    assert_eq!(emu.pc(), auxv[&AT_BASE] + ENTRY_OFFSET as u64);
    assert_eq!(emu.maps.read_string(auxv[&AT_EXECFN]), filename);
    assert_eq!(emu.maps.read_buffer(emu.pc(), handoff.len()), handoff);
    if kind == ET_DYN {
        assert_eq!(
            emu.maps.read_dword(main_base + REL_SLOT as u64),
            Some(ADDEND)
        );
    }
    assert_eq!(emu.run(Some(main_entry)).unwrap(), main_entry);
    if kind == ET_DYN {
        assert_eq!(
            emu.maps.read_dword(main_base + REL_SLOT as u64),
            Some(main_base as u32 + ADDEND)
        );
    }
    let files = emu.linux_files.as_ref().unwrap();
    assert_eq!(files.read_at(image_fd as u32, 0, main.len()), Err(EBADF));
    assert_eq!(
        files.executable_bytes("/proc/self/exe"),
        Ok(main.as_slice())
    );
    assert_eq!(files.executable_bytes(MAIN_PATH), Err(ENOENT));
    assert_eq!(files.attempted_exec(), Some((MAIN_PATH, main.as_slice())));
    assert!(!files.exec_handoff());
}

#[test]
fn getdents64_resumes_after_unlink_and_shares_the_cursor_with_dup() {
    const AT_FDCWD: u64 = -100i64 as u64;
    const CREATE_RDWR: u64 = 0o102;
    const DT_DIR: u8 = 4;
    const DT_REG: u8 = 8;
    const DT_LNK: u8 = 10;
    const SHORT_RECORD: u64 = 24;

    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        emu.set_linux_files(Vec::new(), Vec::new());
        let fs = emu.linux_files.as_mut().unwrap();
        fs.mount_executable("/tmp/sub/nested", b"\x7fELF").unwrap();
        emu.maps.write_string(DATA, "/tmp/file");
        let regular = invoke_with(
            &mut emu,
            Operation::OpenAt,
            [AT_FDCWD, DATA, CREATE_RDWR, 0o600],
        );
        assert!(regular >= 3);
        assert_eq!(
            invoke(
                &mut emu,
                Operation::GetDents64,
                [regular as u64, IOVEC, 256]
            ),
            ENOTDIR as i64
        );
        emu.maps.write_string(DATA, "sub");
        emu.maps.write_string(DATA + 64, "/tmp/link");
        assert_eq!(
            invoke(&mut emu, Operation::SymlinkAt, [DATA, AT_FDCWD, DATA + 64]),
            0
        );
        emu.maps.write_string(DATA, "/tmp");
        let dir = invoke(&mut emu, Operation::OpenAt, [AT_FDCWD, DATA, 0]) as u64;
        let dup = invoke(&mut emu, Operation::DupTo, [dir, 77, 0]) as u64;
        assert_eq!(dup, 77);
        let scan =
            |emu: &mut Emu, fd, addr, bytes| invoke(emu, Operation::GetDents64, [fd, addr, bytes]);
        assert_eq!(scan(&mut emu, dir, IOVEC, 1), EINVAL as i64);
        assert_eq!(scan(&mut emu, dir, 0, SHORT_RECORD), EFAULT as i64);
        let mut last = 0;
        for (name, kind) in [
            (".", DT_DIR),
            ("..", DT_DIR),
            ("sub", DT_DIR),
            ("file", DT_REG),
            ("link", DT_LNK),
        ] {
            assert_eq!(
                scan(&mut emu, dup, IOVEC, SHORT_RECORD),
                SHORT_RECORD as i64
            );
            assert_ne!(emu.maps.read_qword(IOVEC), Some(0));
            let cookie = emu.maps.read_qword(IOVEC + 8).unwrap();
            assert!(cookie > last);
            assert_eq!(
                invoke(&mut emu, Operation::Seek, [dir, 0, 1]),
                cookie as i64
            );
            assert_eq!(emu.maps.read_word(IOVEC + 16), Some(SHORT_RECORD as u16));
            assert_eq!(emu.maps.read_byte(IOVEC + 18), Some(kind));
            assert_eq!(emu.maps.read_string(IOVEC + 19), name);
            if name == "file" {
                emu.maps.write_string(DATA, name);
                assert_eq!(invoke(&mut emu, Operation::UnlinkAt, [dir, DATA, 0]), 0);
            }
            last = cookie;
        }
        assert_eq!(scan(&mut emu, dir, IOVEC, 256), 0);
        assert_eq!(
            invoke(&mut emu, Operation::Seek, [dup, 0, 2]),
            EINVAL as i64
        );
        assert_eq!(invoke(&mut emu, Operation::Seek, [dir, 0, 1]), last as i64);
        assert_eq!(invoke(&mut emu, Operation::Seek, [dup, 0, 0]), 0);
        assert_eq!(
            scan(&mut emu, dir, IOVEC + 4096 - SHORT_RECORD, 256),
            SHORT_RECORD as i64
        );
        assert_eq!(scan(&mut emu, dir, IOVEC, 256), 3 * SHORT_RECORD as i64);
        assert_eq!(emu.maps.read_string(IOVEC + 19), "..");
    }
}

#[test]
fn proc_self_auxv_preserves_the_initial_stack_across_word_sizes() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        emu.set_linux_files(Vec::new(), Vec::new());
        emu.set_sp(IOVEC + 0x1000);
        emu.write_linux_stack_layout(0x401123, 0x400040, 32, 4);
        let word_size = if arch.is_64bits() { 8 } else { 4 };
        // argc=1, argv[0], argv terminator, envp terminator, then the auxiliary vector.
        let address = emu.sp() + 4 * word_size;
        let mut expected = Vec::new();
        for index in 0..64 {
            let pair = emu
                .maps
                .read_bytes(address + 2 * word_size * index, 2 * word_size as usize);
            expected.extend_from_slice(&pair);
            if pair.iter().all(|&byte| byte == 0) {
                break;
            }
        }
        const AT_BASE: u64 = 7;
        let word = word_size as usize;
        let mut entries = expected
            .chunks_exact(2 * word)
            .filter(|pair| pair[..word] == AT_BASE.to_le_bytes()[..word]);
        let base = entries.next().expect("initial stack must contain AT_BASE");
        assert_eq!(&base[word..], &0u64.to_le_bytes()[..word]);
        assert!(entries.next().is_none());
        assert!(emu.maps.write_bytes(address, &vec![0; expected.len()]));
        emu.maps.write_string(DATA, "/proc/self/auxv");
        let fd = invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, DATA, 0]);
        assert!(fd >= 0, "{arch:?} open: {fd}");
        assert_eq!(
            invoke(&mut emu, Operation::Read, [fd as u64, DATA, 1024]),
            expected.len() as i64
        );
        assert_eq!(&*emu.maps.read_bytes(DATA, expected.len()), expected);
    }
}

#[test]
fn standard_streams_preserve_bytes_and_reject_invalid_descriptors() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        assert!(emu.maps.write_bytes(DATA, b"A\0\xffB"));
        for operation in [Operation::Write, Operation::Writev] {
            emu.emulated_stdout.clear();
            vectors(&mut emu, &[(DATA, 2), (DATA + 2, 2)]);
            let (buffer, count) = match operation {
                Operation::Write => (DATA, 4),
                _ => (IOVEC, 2),
            };
            assert_eq!(
                invoke(&mut emu, operation, [STDOUT_FILENO as u64, buffer, count]),
                4,
                "{arch:?} {operation:?}"
            );
            assert_eq!(emu.emulated_stdout, b"A\0\xffB");
            assert_eq!(
                invoke(&mut emu, operation, [STDERR_FILENO as u64, buffer, count]),
                4
            );
            assert_eq!(emu.emulated_stdout, b"A\0\xffB", "stderr is separate");
            for fd in [STDIN_FILENO as u64, u32::MAX as u64] {
                assert_eq!(
                    invoke(&mut emu, operation, [fd, buffer, count]),
                    EBADF as i64
                );
                assert_eq!(emu.emulated_stdout, b"A\0\xffB");
            }
            assert_eq!(invoke(&mut emu, operation, [STDOUT_FILENO as u64, 0, 0]), 0);
        }
        assert_eq!(
            invoke(&mut emu, Operation::Read, [STDIN_FILENO as u64, DATA, 4]),
            0
        );
        for fd in [STDOUT_FILENO as u64, u32::MAX as u64] {
            assert_eq!(
                invoke(&mut emu, Operation::Read, [fd, DATA, 4]),
                EBADF as i64
            );
        }
        assert_eq!(
            emu.maps.read_bytes_option(DATA, 4).unwrap().as_ref(),
            b"A\0\xffB"
        );
        for (fd, mode) in [
            (STDIN_FILENO, O_RDONLY),
            (STDOUT_FILENO, O_WRONLY),
            (STDERR_FILENO, O_WRONLY),
        ] {
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd as u64, F_GETFD as u64, 0]),
                0
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd as u64, F_GETFL as u64, 0]),
                mode as i64
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd as u64, F_SETFD as u64, 0]),
                0
            );
            for command in [F_SETFD, F_SETFL, u32::MAX] {
                assert_eq!(
                    invoke(&mut emu, Operation::Fcntl, [fd as u64, command as u64, 1]),
                    ENOSYS as i64
                );
            }
        }
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Fcntl,
                [u32::MAX as u64, F_GETFD as u64, 0]
            ),
            EBADF as i64
        );
    }
}

#[test]
fn output_reads_validate_iovecs_ranges_and_short_writes() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        for (operation, buffer, count, error) in [
            (Operation::Write, u64::MAX, 2, EFAULT),
            (Operation::Write, DATA + 0xfff, 2, EFAULT),
            (Operation::Writev, IOVEC + 0xfff, 1, EFAULT),
            (Operation::Writev, u64::MAX, 2, EFAULT),
            (Operation::Writev, IOVEC, UIO_MAXIOV as u64 + 1, EINVAL),
        ] {
            assert_eq!(
                invoke(&mut emu, operation, [STDOUT_FILENO as u64, buffer, count]),
                error as i64,
                "{arch:?} {operation:?}"
            );
            assert!(emu.emulated_stdout.is_empty());
        }
        if arch.is_64bits() {
            assert_eq!(
                invoke(
                    &mut emu,
                    Operation::Writev,
                    [STDOUT_FILENO as u64, IOVEC, 1 << 32]
                ),
                EINVAL as i64
            );
        }
        let too_large = if arch.is_64bits() {
            i64::MAX as u64
        } else {
            i32::MAX as u64
        };
        vectors(&mut emu, &[(DATA, too_large), (DATA, 1)]);
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Writev,
                [STDOUT_FILENO as u64, IOVEC, 2]
            ),
            EINVAL as i64
        );
        assert!(emu.emulated_stdout.is_empty());
        assert!(emu.maps.write_bytes(DATA, b"prefix"));
        vectors(&mut emu, &[(DATA, 6), (u32::MAX as u64, 2)]);
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Writev,
                [STDOUT_FILENO as u64, IOVEC, 2]
            ),
            6
        );
        assert_eq!(emu.emulated_stdout, b"prefix");
        emu.emulated_stdout.clear();
        emu.maps
            .create_map("adjacent io", DATA + 0x1000, 0x1000, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_bytes(DATA + 0xffe, b"edge"));
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Write,
                [STDOUT_FILENO as u64, DATA + 0xffe, 4]
            ),
            4
        );
        assert_eq!(emu.emulated_stdout, b"edge");
        emu.emulated_stdout.clear();
        let large = vec![0xa5; 128 * 1024];
        let address = 0x100000;
        emu.maps
            .create_map(
                "large io",
                address,
                large.len() as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        assert!(emu.maps.write_bytes(address, &large));
        let mut written = 0;
        while written < large.len() {
            let result = invoke(
                &mut emu,
                Operation::Write,
                [
                    STDOUT_FILENO as u64,
                    address + written as u64,
                    (large.len() - written) as u64,
                ],
            );
            assert!(result > 0 && result as usize <= large.len() - written);
            written += result as usize;
        }
        assert_eq!(emu.emulated_stdout, large);
    }
}

fn pipe(emu: &mut Emu) -> (u64, u64) {
    assert_eq!(
        invoke(
            emu,
            Operation::Pipe2,
            [IOVEC, (O_NONBLOCK | O_CLOEXEC) as u64, 0]
        ),
        0
    );
    (
        emu.maps.read_dword(IOVEC).unwrap() as u64,
        emu.maps.read_dword(IOVEC + 4).unwrap() as u64,
    )
}

#[test]
fn pipe_queues_preserve_data_and_report_peer_lifetime() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        for (output, flags, error) in [(0, O_NONBLOCK, EFAULT), (IOVEC, u32::MAX, EINVAL)] {
            assert_eq!(
                invoke(&mut emu, Operation::Pipe2, [output, flags as u64, 0]),
                error as i64
            );
        }
        let (reader, writer) = pipe(&mut emu);
        for (fd, mode) in [(reader, O_RDONLY), (writer, O_WRONLY)] {
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_SETFD as u64, FD_CLOEXEC]),
                0
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_SETFD as u64, 0]),
                ENOSYS as i64
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_GETFD as u64, 0]),
                FD_CLOEXEC as i64
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_GETFL as u64, 0]),
                (mode | O_NONBLOCK as u64) as i64
            );
        }
        assert_eq!(
            invoke(&mut emu, Operation::Read, [reader, DATA, 4]),
            EAGAIN as i64
        );
        assert_eq!(
            invoke(&mut emu, Operation::Read, [writer, DATA, 4]),
            EBADF as i64
        );
        assert_eq!(
            invoke(&mut emu, Operation::Write, [reader, DATA, 4]),
            EBADF as i64
        );
        assert!(emu.maps.write_bytes(DATA, b"A\0\xffB"));
        assert_eq!(invoke(&mut emu, Operation::Write, [writer, DATA, 4]), 4);
        assert_eq!(
            invoke(&mut emu, Operation::Read, [reader, 0, 4]),
            EFAULT as i64
        );
        assert_eq!(invoke(&mut emu, Operation::Read, [reader, DATA + 16, 2]), 2);
        assert_eq!(
            emu.maps.read_bytes_option(DATA + 16, 2).unwrap().as_ref(),
            b"A\0"
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [writer, 0, 0]), 0);
        assert_eq!(invoke(&mut emu, Operation::Read, [reader, DATA + 16, 4]), 2);
        assert_eq!(
            emu.maps.read_bytes_option(DATA + 16, 2).unwrap().as_ref(),
            b"\xffB"
        );
        assert_eq!(invoke(&mut emu, Operation::Read, [reader, DATA, 1]), 0);
        assert_eq!(invoke(&mut emu, Operation::Close, [reader, 0, 0]), 0);

        let (reader, writer) = pipe(&mut emu);
        let capacity = 64 * 1024;
        let bulk = 0x100000;
        emu.maps
            .create_map("pipe capacity", bulk, capacity, Permission::READ_WRITE)
            .unwrap();
        assert_eq!(
            invoke(&mut emu, Operation::Write, [writer, bulk, capacity - 1]),
            (capacity - 1) as i64
        );
        assert_eq!(
            invoke(&mut emu, Operation::Write, [writer, DATA, 2]),
            EAGAIN as i64,
            "atomic small write"
        );
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Write,
                [writer, bulk, PIPE_BUF as u64 + 1]
            ),
            1,
            "large write may be partial"
        );
        assert_eq!(
            invoke(&mut emu, Operation::Write, [writer, DATA, 1]),
            EAGAIN as i64
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [reader, 0, 0]), 0);
        assert_eq!(
            invoke(&mut emu, Operation::Close, [reader, 0, 0]),
            EBADF as i64
        );
        assert_eq!(
            invoke(&mut emu, Operation::Write, [writer, DATA, 1]),
            EPIPE as i64
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [writer, 0, 0]), 0);
    }
}

#[test]
fn event_counters_validate_width_overflow_and_semaphore_reads() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        assert_eq!(
            invoke(&mut emu, Operation::Eventfd2, [0, u32::MAX as u64, 0]),
            EINVAL as i64
        );
        for semaphore in [false, true] {
            let flags = EFD_NONBLOCK | EFD_CLOEXEC | if semaphore { EFD_SEMAPHORE } else { 0 };
            let result = invoke(&mut emu, Operation::Eventfd2, [2, flags as u64, 0]);
            assert!(result > 2, "{arch:?}");
            let fd = result as u64;
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_GETFL as u64, 0]),
                (O_RDWR | O_NONBLOCK as u64) as i64
            );
            assert_eq!(
                invoke(&mut emu, Operation::Fcntl, [fd, F_GETFD as u64, 0]),
                FD_CLOEXEC as i64
            );
            for count in [0, 7, 16] {
                assert_eq!(
                    invoke(&mut emu, Operation::Write, [fd, DATA, count]),
                    EINVAL as i64
                );
            }
            assert_eq!(
                invoke(&mut emu, Operation::Write, [fd, 0, 8]),
                EFAULT as i64
            );
            assert_eq!(
                invoke(&mut emu, Operation::Read, [fd, DATA, 7]),
                EINVAL as i64
            );
            for expected in if semaphore { vec![1, 1] } else { vec![2] } {
                assert_eq!(invoke(&mut emu, Operation::Read, [fd, DATA, 16]), 8);
                assert_eq!(emu.maps.read_qword(DATA), Some(expected));
            }
            assert_eq!(
                invoke(&mut emu, Operation::Read, [fd, DATA, 8]),
                EAGAIN as i64
            );
            assert!(emu.maps.write_qword(DATA, u64::MAX));
            assert_eq!(
                invoke(&mut emu, Operation::Write, [fd, DATA, 8]),
                EINVAL as i64
            );
            assert!(emu.maps.write_qword(DATA, u64::MAX - 1));
            assert_eq!(invoke(&mut emu, Operation::Write, [fd, DATA, 8]), 8);
            assert!(emu.maps.write_qword(DATA, 1));
            assert_eq!(
                invoke(&mut emu, Operation::Write, [fd, DATA, 8]),
                EAGAIN as i64
            );
            assert_eq!(invoke(&mut emu, Operation::Close, [fd, 0, 0]), 0);
        }
        let fd = invoke(&mut emu, Operation::Eventfd2, [0, EFD_NONBLOCK as u64, 0]) as u64;
        assert!(emu.maps.write_qword(DATA, 3));
        vectors(&mut emu, &[(DATA, 8), (DATA, 8)]);
        assert_eq!(invoke(&mut emu, Operation::Writev, [fd, IOVEC, 2]), 16);
        assert_eq!(invoke(&mut emu, Operation::Read, [fd, DATA + 16, 8]), 8);
        assert_eq!(emu.maps.read_qword(DATA + 16), Some(6));
        vectors(&mut emu, &[(DATA, 4), (DATA + 4, 4)]);
        assert_eq!(
            invoke(&mut emu, Operation::Writev, [fd, IOVEC, 2]),
            EINVAL as i64
        );
        vectors(&mut emu, &[(DATA, 8), (0, 8)]);
        assert_eq!(invoke(&mut emu, Operation::Writev, [fd, IOVEC, 2]), 8);
        assert_eq!(invoke(&mut emu, Operation::Read, [fd, 0, 8]), EFAULT as i64);
        assert_eq!(
            invoke(&mut emu, Operation::Read, [fd, DATA, 8]),
            EAGAIN as i64,
            "Linux consumes the counter even when the copy fails"
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [fd, 0, 0]), 0);
    }
}

#[test]
fn libc_and_raw_syscalls_share_descriptors_and_translate_errno() {
    fn library_call(emu: &mut Emu, name: &str, args: [u64; 3]) -> i64 {
        if emu.cfg.arch.is_aarch64() {
            emu.regs_aarch64_mut().x[..args.len()].copy_from_slice(&args);
            libc::gateway(name, emu);
            emu.regs_aarch64().x[0] as i64
        } else {
            let regs = emu.regs_mut();
            [regs.rdi, regs.rsi, regs.rdx] = args;
            libc::gateway(name, emu);
            emu.regs().rax as i64
        }
    }
    for arch in [Arch::X86_64, Arch::Aarch64] {
        let mut emu = environment(arch);
        let errno = unix::errno_address(&mut emu).unwrap();
        assert!(emu.maps.write_dword(errno, 123));
        assert_eq!(invoke(&mut emu, Operation::Pipe2, [IOVEC, 0, 0]), 0);
        let reader = emu.maps.read_dword(IOVEC).unwrap() as u64;
        let writer = emu.maps.read_dword(IOVEC + 4).unwrap() as u64;
        assert!(emu.maps.write_bytes(DATA, b"x\xff\0"));
        assert_eq!(library_call(&mut emu, "__write", [writer, DATA, 2]), 2);
        assert_eq!(library_call(&mut emu, "read", [reader, DATA + 4, 8]), 2);
        assert_eq!(emu.maps.read_word(DATA + 4), Some(0xff78));
        assert_eq!(library_call(&mut emu, "__close", [writer, 0, 0]), 0);
        assert_eq!(library_call(&mut emu, "__read", [reader, DATA, 1]), 0);
        assert_eq!(invoke(&mut emu, Operation::Close, [reader, 0, 0]), 0);
        assert_eq!(emu.maps.read_dword(errno), Some(123));
        for (name, args, error) in [
            ("close", [u32::MAX as u64, 0, 0], EBADF),
            ("read", [u32::MAX as u64, DATA, 1], EBADF),
            ("write", [1, 0, 1], EFAULT),
            ("open", [0, 0, 0], EFAULT),
            ("__open64", [DATA + 2, 0, 0], ENOENT),
            ("__open", [DATA, 0, 0], ENOSYS),
            ("open64", [DATA, 0, 0], ENOSYS),
        ] {
            assert_eq!(library_call(&mut emu, name, args), -1, "{arch:?} {name}");
            assert_eq!(
                emu.maps.read_dword(errno),
                Some(error.wrapping_neg() as u32)
            );
        }
        assert_eq!(library_call(&mut emu, "write", [2, DATA, 2]), 2);
        assert_eq!(emu.emulated_stdout, b"x\xff");
        emu.set_linux_files(b"content".to_vec(), b"in".to_vec());
        emu.maps.write_string(DATA, "/proc/self/exe");
        let fd = library_call(&mut emu, "open", [DATA, 0, 0]);
        assert!(fd >= 3);
        assert_eq!(invoke(&mut emu, Operation::Read, [fd as u64, IOVEC, 3]), 3);
        assert_eq!(library_call(&mut emu, "read", [fd as u64, IOVEC, 4]), 4);
        assert_eq!(emu.maps.read_buffer(IOVEC, 4), b"tent");
        assert_eq!(library_call(&mut emu, "close", [fd as u64, 0, 0]), 0);
    }
}

#[test]
fn guest_files_preserve_offsets_stdin_and_memfd_contents_on_all_linux_abis() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        emu.set_linux_files(b"mapped input".to_vec(), b"line".to_vec());
        assert_eq!(invoke(&mut emu, Operation::Read, [0, DATA, 2]), 2);
        assert_eq!(emu.maps.read_word(DATA), Some(u16::from_le_bytes(*b"li")));
        assert_eq!(invoke(&mut emu, Operation::Read, [0, DATA, 8]), 2);
        assert_eq!(invoke(&mut emu, Operation::Read, [0, DATA, 8]), 0);
        for (path, prefix) in [
            ("/proc/self/exe", &b"mapped"[..]),
            ("/proc/self/maps", b"00080000"),
        ] {
            emu.maps.write_string(DATA, path);
            let fd = invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, DATA, 0]);
            assert!(fd >= 3);
            let fd = fd as u64;
            let n = invoke(&mut emu, Operation::Read, [fd, IOVEC, 64]);
            assert!(n >= prefix.len() as i64);
            if path == "/proc/self/exe" {
                assert_eq!(emu.maps.read_buffer(IOVEC, prefix.len()), prefix);
            } else {
                let content = emu.maps.read_buffer(IOVEC, n as usize);
                assert!(content.contains(&b'-'));
            }
            assert_eq!(invoke(&mut emu, Operation::Close, [fd, 0, 0]), 0);
        }
        emu.maps.write_string(DATA, "fixture");
        let fd = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]);
        assert!(fd >= 3);
        let fd = fd as u64;
        assert_eq!(invoke(&mut emu, Operation::Write, [fd, DATA, 7]), 7);
        assert_eq!(invoke(&mut emu, Operation::Seek, [fd, 0, 0]), 0);
        assert_eq!(invoke(&mut emu, Operation::Read, [fd, IOVEC, 9]), 7);
        assert_eq!(emu.maps.read_buffer(IOVEC, 7), b"fixture");
        emu.maps.write_string(DATA, "/etc/passwd");
        assert_eq!(
            invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, DATA, 0]),
            ENOENT as i64
        );
    }
}

#[test]
fn guest_proc_fd_follows_the_open_inode_after_unlink() {
    const DIR: u64 = (-100i64) as u64;
    const PATH: &str = "/tmp/guest-image";
    const CREATE_RDWR: u64 = 0o102;
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        emu.set_linux_files(Vec::new(), Vec::new());
        emu.maps.write_string(DATA, PATH);
        let fd = invoke_with(&mut emu, Operation::OpenAt, [DIR, DATA, CREATE_RDWR, 0o700]);
        assert!(fd >= 3);
        emu.maps.write_bytes(IOVEC, b"old");
        assert_eq!(invoke(&mut emu, Operation::Write, [fd as u64, IOVEC, 3]), 3);
        assert_eq!(invoke(&mut emu, Operation::UnlinkAt, [DIR, DATA, 0]), 0);
        assert_eq!(
            invoke(&mut emu, Operation::AccessAt, [DIR, DATA, 0]),
            ENOENT as i64
        );
        let replacement = invoke_with(&mut emu, Operation::OpenAt, [DIR, DATA, CREATE_RDWR, 0o600]);
        assert!(replacement >= 3, "open replacement: {replacement}");
        assert_ne!(replacement, fd);

        let pid = emu.processes.pid;
        for process in ["self".to_owned(), pid.to_string()] {
            emu.maps
                .write_string(DATA, &format!("/proc/{process}/fd/{fd}"));
            assert_eq!(invoke(&mut emu, Operation::AccessAt, [DIR, DATA, 1]), 0);
            let path = format!("{PATH} (deleted)");
            assert_eq!(
                invoke_with(&mut emu, Operation::ReadlinkAt, [DIR, DATA, IOVEC, 128]),
                path.len() as i64
            );
            assert_eq!(emu.maps.read_buffer(IOVEC, path.len()), path.as_bytes());
            let reopened = invoke(&mut emu, Operation::OpenAt, [DIR, DATA, 0]);
            assert!(
                reopened >= 3,
                "{arch:?} reopening /proc/{process}/fd/{fd}: {reopened}"
            );
            assert_ne!(reopened, replacement);
            assert_ne!(reopened, fd);
            assert_eq!(
                invoke(&mut emu, Operation::Read, [reopened as u64, IOVEC, 3]),
                3
            );
            assert_eq!(emu.maps.read_buffer(IOVEC, 3), b"old");
            assert_eq!(
                invoke(&mut emu, Operation::Close, [reopened as u64, 0, 0]),
                0
            );
        }
        emu.maps
            .write_string(DATA, &format!("/proc/{}/fd/{fd}", pid + 1));
        assert_eq!(
            invoke(&mut emu, Operation::OpenAt, [DIR, DATA, 0]),
            ENOENT as i64
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [fd as u64, 0, 0]), 0);
        emu.maps.write_string(DATA, &format!("/proc/{pid}/fd/{fd}"));
        assert_eq!(
            invoke(&mut emu, Operation::AccessAt, [DIR, DATA, 0]),
            ENOENT as i64
        );
    }
}

#[test]
fn failed_guest_exec_preserves_the_caller_and_rejects_invalid_images() {
    const IMAGE: &[u8] = include_bytes!("../fixtures/elf64_x86_64_linux_hello.bin");
    const BYTES: u64 = 0x200000;
    for (arch, image, expected) in [
        (Arch::X86_64, IMAGE, ENOENT),
        (Arch::X86, IMAGE, ENOENT),
        (Arch::Aarch64, IMAGE, ENOEXEC),
        (Arch::X86_64, &IMAGE[..16], ENOEXEC),
    ] {
        let mut emu = environment(arch);
        emu.set_linux_files(Vec::new(), Vec::new());
        emu.maps.write_string(DATA, "exec");
        let fd = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]);
        assert!(fd >= 3);
        emu.maps
            .create_map(
                "guest image",
                BYTES,
                image.len() as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        assert!(emu.maps.write_bytes(BYTES, image));
        assert_eq!(
            invoke(
                &mut emu,
                Operation::Write,
                [fd as u64, BYTES, image.len() as u64]
            ),
            image.len() as i64
        );
        emu.maps
            .write_string(DATA, &format!("/proc/{}/fd/{fd}", emu.processes.pid));
        assert_eq!(
            invoke(&mut emu, Operation::Exec, [DATA, 0, 0]),
            expected as i64
        );
        let files = emu.linux_files.as_ref().unwrap();
        assert_eq!(emu.cfg.arch, arch);
        assert!(!files.exec_handoff());
        assert!(files.attempted_exec().is_none());
        assert!(files.produced_files().any(|(_, bytes)| bytes == image));
    }
}

#[test]
fn child_redirection_shares_open_offsets_without_replacing_parent_descriptors() {
    for arch in ARCHITECTURES {
        let mut emu = environment(arch);
        emu.set_linux_files(Vec::new(), Vec::new());
        let files = emu.linux_files.as_mut().unwrap();
        files
            .mount_executable("/bin/example", b"\x7fELFtest")
            .unwrap();
        assert_eq!(
            files.mount_executable("/bin/example", b"\x7fELFtest"),
            Err(crate::syscall::linux::constants::errors::EEXIST)
        );
        emu.maps.write_bytes(DATA, b"result\0");
        let fd = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]) as u64;
        emu.maps.write_bytes(IOVEC, b"parent");
        emu.save_linux_auxv(IOVEC, 6);
        let saved = emu.linux_files.as_mut().unwrap().begin_child();
        emu.maps.write_bytes(IOVEC, b"child");
        emu.save_linux_auxv(IOVEC, 5);
        assert_eq!(invoke(&mut emu, Operation::DupTo, [fd, 1, 0]), 1);
        assert_eq!(invoke(&mut emu, Operation::Close, [fd, 0, 0]), 0);
        assert_eq!(invoke(&mut emu, Operation::Write, [1, DATA, 6]), 6);
        assert!(emu.emulated_stdout.is_empty());
        emu.maps.write_bytes(IOVEC, b"/dev/stdout\0");
        let alias = invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, IOVEC, 0]) as u64;
        assert_eq!(invoke(&mut emu, Operation::Read, [alias, IOVEC, 6]), 6);
        assert_eq!(emu.maps.read_buffer(IOVEC, 6), b"result");
        emu.linux_files.as_mut().unwrap().restore_parent(saved);
        assert_eq!(invoke(&mut emu, Operation::Seek, [fd, 0, 1]), 6);
        assert_eq!(
            emu.linux_files.as_ref().unwrap().read_at(fd as u32, 0, 6),
            Ok(&b"result"[..])
        );
        assert_eq!(invoke(&mut emu, Operation::Write, [1, DATA, 6]), 6);
        assert_eq!(emu.emulated_stdout, b"result");
        emu.maps.write_bytes(DATA, b"/proc/self/auxv\0");
        let dir = (-100i64) as u64;
        let auxv = invoke(&mut emu, Operation::OpenAt, [dir, DATA, 0]) as u64;
        assert_eq!(
            emu.linux_files.as_ref().unwrap().read_at(auxv as u32, 0, 6),
            Ok(&b"parent"[..])
        );
        assert_eq!(invoke(&mut emu, Operation::Close, [1, 0, 0]), 0);
        assert_eq!(
            invoke(&mut emu, Operation::Write, [1, DATA, 6]),
            EBADF as i64
        );
    }
}

#[test]
fn guest_file_write_larger_than_one_chunk_preserves_the_tail() {
    const ADDRESS: u64 = 0x200000;
    const LENGTH: u64 = 1024 * 1024 + 9;
    let mut emu = environment(Arch::X86_64);
    emu.set_linux_files(Vec::new(), Vec::new());
    emu.maps
        .create_map("large write", ADDRESS, LENGTH, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_bytes(ADDRESS + LENGTH - 4, b"tail");
    emu.maps.write_bytes(DATA, b"test\0");
    let fd = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]) as u64;
    assert_eq!(
        invoke(&mut emu, Operation::Write, [fd, ADDRESS, LENGTH]),
        LENGTH as i64
    );
    assert_eq!(
        emu.linux_files
            .as_ref()
            .unwrap()
            .read_at(fd as u32, LENGTH - 4, 4),
        Ok(&b"tail"[..])
    );
}

#[test]
fn last_close_retains_anonymous_elf_only_for_the_observer() {
    let mut emu = environment(Arch::X86_64);
    emu.set_linux_files(Vec::new(), Vec::new());
    emu.maps.write_bytes(DATA, b"guest\0");
    let fd = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]) as u64;
    let payload = b"\x7fELFguest";
    let len = payload.len() as u64;
    emu.maps.write_bytes(DATA, payload);
    assert_eq!(
        invoke(&mut emu, Operation::Write, [fd, DATA, len]),
        len as i64
    );
    assert_eq!(invoke(&mut emu, Operation::Close, [fd, 0, 0]), 0);
    let files = emu.linux_files.as_ref().unwrap();
    assert_eq!(files.read_at(fd as u32, 0, payload.len()), Err(EBADF));
    assert!(files.produced_files().any(|(_, data)| data == payload));
}

#[test]
fn file_and_pipe_allocations_respect_each_others_descriptors() {
    let mut emu = environment(Arch::X86);
    emu.set_linux_files(Vec::new(), Vec::new());
    emu.maps.write_bytes(DATA, b"one\0");
    let first = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]) as u32;
    assert_eq!(invoke(&mut emu, Operation::Pipe2, [IOVEC, 0, 0]), 0);
    let reader = emu.maps.read_dword(IOVEC).unwrap();
    let writer = emu.maps.read_dword(IOVEC + 4).unwrap();
    let second = invoke(&mut emu, Operation::Memfd, [DATA, 0, 0]) as u32;
    let unique: std::collections::BTreeSet<_> =
        [first, reader, writer, second].into_iter().collect();
    assert_eq!(unique.len(), 4);
    let mut other = environment(Arch::X86);
    other.set_linux_files(Vec::new(), Vec::new());
    assert_eq!(
        invoke(&mut other, Operation::Close, [writer as u64, 0, 0]),
        EBADF as i64
    );
    let saved = emu.linux_files.as_mut().unwrap().begin_child();
    assert_eq!(invoke(&mut emu, Operation::Close, [writer as u64, 0, 0]), 0);
    emu.linux_files.as_mut().unwrap().restore_parent(saved);
    assert_eq!(
        invoke(&mut emu, Operation::Write, [writer as u64, DATA, 3]),
        3
    );
    assert_eq!(
        invoke(&mut emu, Operation::Read, [reader as u64, IOVEC, 3]),
        3
    );
    assert_eq!(emu.maps.read_buffer(IOVEC, 3), b"one");
    for fd in unique {
        assert_eq!(invoke(&mut emu, Operation::Close, [fd as u64, 0, 0]), 0);
    }
    assert_eq!(invoke(&mut emu, Operation::Close, [0, 0, 0]), 0);
    assert_eq!(
        invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, DATA, 0]),
        ENOENT as i64
    );
    emu.maps.write_bytes(DATA, b"/dev/null\0");
    assert_eq!(
        invoke(&mut emu, Operation::OpenAt, [(-100i64) as u64, DATA, 0]),
        0
    );
}
