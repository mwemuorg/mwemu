use crate::api::{linux, macos};
use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::syscall::linux::constants::errors::ENOSYS;
use crate::syscall::linux::constants::syscalls::{i386_syscall_name, x86_64_syscall_name};
use crate::syscall::linux::{syscall_aarch64, syscall32, syscall64};
use crate::tests::helpers;
use crate::threading::context::ThreadContext;
use crate::threading::scheduler::ThreadScheduler;
use crate::{emu_aarch64, emu32, emu64};
use itertools::iproduct;

fn errno_pointer(emu: &mut Emu) -> u64 {
    if emu.os == OperatingSystem::MacOS {
        macos::gateway(emu.pc(), "libSystem.B.dylib", "___error", emu);
    } else {
        linux::gateway(emu.pc(), "libc.so.6", "__errno_location", emu);
    }
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64().x[0]
    } else {
        emu.regs().rax
    }
}

#[test]
fn errno_storage_is_stable_per_thread_and_survives_snapshots() {
    helpers::setup();
    for (os, arm) in iproduct!(
        [OperatingSystem::Linux, OperatingSystem::MacOS],
        [false, true]
    ) {
        let mut emu = if arm { emu_aarch64() } else { emu64() };
        emu.os = os;
        let parent = errno_pointer(&mut emu);
        assert_ne!(parent, 0);
        assert_eq!(emu.maps.read_dword(parent), Some(0));
        assert!(emu.maps.write_dword(parent, 17));
        let maps = emu.maps.maps.len();
        assert_eq!(errno_pointer(&mut emu), parent);
        assert_eq!(emu.maps.maps.len(), maps);
        assert_eq!(emu.maps.read_dword(parent), Some(17));
        emu.threads.push(ThreadContext::new(2, emu.cfg.arch));
        assert!(ThreadScheduler::switch_to_thread(&mut emu, 1));
        let child = errno_pointer(&mut emu);
        assert_ne!(parent, child);
        assert_eq!(emu.maps.read_dword(child), Some(0));
        assert!(emu.maps.write_dword(child, 34));
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        for (thread, address, value) in [(0, parent, 17), (1, child, 34)] {
            assert!(ThreadScheduler::switch_to_thread(&mut emu, thread));
            assert_eq!(errno_pointer(&mut emu), address);
            assert_eq!(emu.maps.read_dword(address), Some(value));
        }
        let mut separate = if arm { emu_aarch64() } else { emu64() };
        separate.os = os;
        let address = errno_pointer(&mut separate);
        assert_eq!(separate.maps.read_dword(address), Some(0));
    }
}

#[derive(Clone, Copy, Debug)]
enum IdentityCall {
    Process,
    Parent,
    Thread,
    ClearTid,
}

impl IdentityCall {
    fn number(self, arch: Arch) -> u64 {
        let numbers = match self {
            Self::Process => [20, 39, 172],
            Self::Parent => [64, 110, 173],
            Self::Thread => [224, 186, 178],
            Self::ClearTid => [258, 218, 96],
        };
        numbers[match arch {
            Arch::X86 => 0,
            Arch::X86_64 => 1,
            Arch::Aarch64 => 2,
        }]
    }
}

fn linux_identity(emu: &mut Emu, operation: IdentityCall, address: u64) -> u64 {
    let number = operation.number(emu.cfg.arch);
    match emu.cfg.arch {
        Arch::X86 => {
            emu.regs_mut().rax = number;
            emu.regs_mut().rbx = address;
            syscall32::gateway(emu);
            emu.regs().get_eax()
        }
        Arch::X86_64 => {
            emu.regs_mut().rax = number;
            emu.regs_mut().rdi = address;
            syscall64::gateway(emu);
            emu.regs().rax
        }
        Arch::Aarch64 => {
            emu.regs_aarch64_mut().x[8] = number;
            emu.regs_aarch64_mut().x[0] = address;
            syscall_aarch64::gateway(emu);
            emu.regs_aarch64().x[0]
        }
    }
}

fn unix_export(emu: &mut Emu, name: &str) -> u64 {
    if emu.os == OperatingSystem::MacOS {
        macos::gateway(emu.pc(), "libSystem.B.dylib", &format!("_{name}"), emu);
    } else {
        linux::gateway(emu.pc(), "libc.so.6", name, emu);
    }
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64().x[0]
    } else {
        emu.regs().rax
    }
}

#[test]
fn unix_process_identity_tracks_fork_and_parent_restoration() {
    helpers::setup();
    for (os, arm) in iproduct!(
        [OperatingSystem::Linux, OperatingSystem::MacOS],
        [false, true]
    ) {
        let mut emu = if arm { emu_aarch64() } else { emu64() };
        emu.os = os;
        let parent = emu.processes.pid;
        let grandparent = emu.processes.ppid;
        assert_eq!(unix_export(&mut emu, "getpid"), parent);
        assert_eq!(unix_export(&mut emu, "getppid"), grandparent);
        let child = emu.fork_begin();
        assert_ne!(child, parent);
        assert_eq!(unix_export(&mut emu, "getpid"), child);
        assert_eq!(unix_export(&mut emu, "getppid"), parent);
        assert_eq!(emu.process_exit(0), Some(child));
        assert_eq!(unix_export(&mut emu, "getpid"), parent);
        assert_eq!(unix_export(&mut emu, "getppid"), grandparent);
    }
}

#[test]
fn linux_identity_matches_process_threads_and_clear_tid_registration() {
    helpers::setup();
    const SECONDARY_TID: u64 = 0x1001;
    const TID_ADDRESS: u64 = 0x80000;
    for arch in [Arch::X86, Arch::X86_64, Arch::Aarch64] {
        let mut emu = match arch {
            Arch::X86 => emu32(),
            Arch::X86_64 => emu64(),
            Arch::Aarch64 => emu_aarch64(),
        };
        emu.os = OperatingSystem::Linux;
        let pid = emu.processes.pid;
        let ppid = emu.processes.ppid;
        emu.maps
            .create_map("identity", TID_ADDRESS, 4, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_dword(TID_ADDRESS, 0xfeed_beef));
        emu.threads.push(ThreadContext::new(SECONDARY_TID, arch));
        for (thread, tid) in [(0, pid), (1, SECONDARY_TID)] {
            assert!(ThreadScheduler::switch_to_thread(&mut emu, thread));
            for (operation, expected) in [
                (IdentityCall::Process, pid),
                (IdentityCall::Parent, ppid),
                (IdentityCall::Thread, tid),
            ] {
                assert_eq!(
                    linux_identity(&mut emu, operation, 0),
                    expected,
                    "{arch:?}: {operation:?}"
                );
            }
            if arch != Arch::X86 {
                assert_eq!(unix_export(&mut emu, "getpid"), pid);
                assert_eq!(unix_export(&mut emu, "gettid"), tid);
            }
            for address in [TID_ADDRESS, 0, TID_ADDRESS + 4] {
                assert_eq!(
                    linux_identity(&mut emu, IdentityCall::ClearTid, address),
                    tid
                );
                assert_eq!(emu.threads[thread].clear_child_tid, address);
                assert_eq!(emu.maps.read_dword(TID_ADDRESS), Some(0xfeed_beef));
            }
        }
        let child = emu.fork_begin();
        assert_eq!(linux_identity(&mut emu, IdentityCall::Process, 0), child);
        assert_eq!(linux_identity(&mut emu, IdentityCall::Thread, 0), child);
        assert_eq!(linux_identity(&mut emu, IdentityCall::Parent, 0), pid);
        assert_eq!(emu.process_exit(0), Some(child));
        assert_eq!(
            linux_identity(&mut emu, IdentityCall::Thread, 0),
            SECONDARY_TID
        );
    }
}

#[test]
fn integer_conversions_preserve_range_end_pointer_and_thread_errno() {
    helpers::setup();
    for (os, arm) in iproduct!(
        [OperatingSystem::Linux, OperatingSystem::MacOS],
        [false, true]
    ) {
        let mut emu = if arm { emu_aarch64() } else { emu64() };
        emu.os = os;
        let input = emu.maps.alloc(512).unwrap();
        emu.maps
            .create_map("conversion", input, 512, Permission::READ_WRITE)
            .unwrap();
        let end = input + 256;
        let errno = errno_pointer(&mut emu);
        let marker = 17;
        for (symbol, text, base, result, consumed, expected_errno) in [
            ("strtol", "  -0x1Fz", 0, (-31_i64) as u64, Some(7), marker),
            ("strtoul", "0755", 0, 0o755, Some(4), marker),
            ("strtoll", "0xz", 16, 0, Some(1), marker),
            (
                "strtoll",
                "-9223372036854775808",
                10,
                i64::MIN as u64,
                Some(20),
                marker,
            ),
            (
                "strtol",
                "-9223372036854775809x",
                10,
                i64::MIN as u64,
                Some(20),
                34,
            ),
            (
                "strtoimax",
                "9223372036854775808!",
                10,
                i64::MAX as u64,
                Some(19),
                34,
            ),
            (
                "strtoumax",
                "18446744073709551615",
                10,
                u64::MAX,
                Some(20),
                marker,
            ),
            (
                "strtoull",
                "-18446744073709551616",
                10,
                u64::MAX,
                Some(21),
                34,
            ),
            ("strtoul", "-2", 10, u64::MAX - 1, Some(2), marker),
            (
                "strtol",
                "bad",
                10,
                0,
                Some(0),
                if os == OperatingSystem::MacOS {
                    22
                } else {
                    marker
                },
            ),
            (
                "strtol",
                "42",
                1,
                0,
                if os == OperatingSystem::MacOS {
                    Some(0)
                } else {
                    None
                },
                22,
            ),
            ("atoi", "42xyz", 10, 42, None, marker),
        ] {
            emu.maps.write_string(input, text);
            emu.maps.write_qword(end, u64::MAX);
            emu.maps.write_dword(errno, marker);
            if arm {
                emu.regs_aarch64_mut().x[..3].copy_from_slice(&[input, end, base]);
            } else {
                let regs = emu.regs_mut();
                regs.rdi = input;
                regs.rsi = end;
                regs.rdx = base;
            }
            assert_eq!(
                unix_export(&mut emu, symbol),
                result,
                "{os:?} {symbol}({text})"
            );
            assert_eq!(
                emu.maps.read_qword(end),
                Some(consumed.map_or(u64::MAX, |n| input + n))
            );
            assert_eq!(emu.maps.read_dword(errno), Some(expected_errno));
        }
    }
}

#[test]
fn libc_page_queries_and_unknown_names_use_the_guest_profile() {
    use crate::maps::virtual_memory::PAGE_SIZE;
    helpers::setup();
    for (os, arm) in iproduct!(
        [OperatingSystem::Linux, OperatingSystem::MacOS],
        [false, true]
    ) {
        let mut emu = if arm { emu_aarch64() } else { emu64() };
        emu.os = os;
        let errno = errno_pointer(&mut emu);
        assert_eq!(unix_export(&mut emu, "getpagesize"), PAGE_SIZE);
        for (query, value, error) in [
            (if os.is_linux() { 30 } else { 29 }, PAGE_SIZE, 7),
            (u64::MAX, u64::MAX, 22),
        ] {
            assert!(emu.maps.write_dword(errno, 7));
            if arm {
                emu.regs_aarch64_mut().x[0] = query;
            } else {
                emu.regs_mut().rdi = query;
            }
            assert_eq!(unix_export(&mut emu, "sysconf"), value);
            assert_eq!(emu.maps.read_dword(errno), Some(error));
        }
    }
}

#[test]
fn x86_syscall_names_follow_the_sparse_abi_tables() {
    for (number, name) in [
        (221, Some("fcntl64")),
        (222, None),
        (223, None),
        (224, Some("gettid")),
        (386, Some("rseq")),
        (392, None),
        (393, Some("semget")),
        (448, Some("process_mrelease")),
        (u32::MAX as u64, None),
    ] {
        assert_eq!(i386_syscall_name(number), name, "i386 syscall {number}");
    }

    for (number, name) in [
        (334, Some("rseq")),
        (400, None),
        (423, None),
        (424, Some("pidfd_send_signal")),
        (448, Some("process_mrelease")),
        (u64::MAX, None),
    ] {
        assert_eq!(x86_64_syscall_name(number), name, "x86_64 syscall {number}");
    }
}

fn raw_linux_syscall(emu: &mut Emu, number: u64, args: [u64; 3]) -> u64 {
    let [a, b, c] = args;
    match emu.cfg.arch {
        Arch::X86 => {
            let r = emu.regs_mut();
            [r.rax, r.rbx, r.rcx, r.rdx] = [number, a, b, c];
            syscall32::gateway(emu);
            emu.regs().get_eax() as i32 as i64 as u64
        }
        Arch::X86_64 => {
            let r = emu.regs_mut();
            [r.rax, r.rdi, r.rsi, r.rdx] = [number, a, b, c];
            syscall64::gateway(emu);
            emu.regs().rax
        }
        Arch::Aarch64 => {
            let r = emu.regs_aarch64_mut();
            r.x[8] = number;
            r.x[..3].copy_from_slice(&args);
            syscall_aarch64::gateway(emu);
            emu.regs_aarch64().x[0]
        }
    }
}

#[test]
fn amd64_fork_resumes_the_parent_and_reaps_the_child_once() {
    use crate::syscall::linux::constants::errors::{ECHILD, EFAULT};
    use crate::syscall::linux::constants::syscalls::{NR64_EXIT_GROUP, NR64_FORK, NR64_WAIT4};

    const CODE: u64 = 0x400000;
    const DATA: u64 = 0x500000;
    const STATUS: u64 = DATA + 4;
    const CHILD_STATUS: u64 = 7;
    const PREFIXED_SYSCALL: &[u8] = &[0x48, 0x0f, 0x05];

    helpers::setup();
    let mut emu = emu64();
    emu.os = OperatingSystem::Linux;
    emu.maps
        .create_map("syscall", CODE, 16, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    emu.maps
        .create_map("data", DATA, 16, Permission::READ_WRITE)
        .unwrap();
    assert!(emu.maps.write_bytes(CODE, PREFIXED_SYSCALL));
    emu.set_pc(CODE);
    let parent = emu.processes.pid;
    assert_eq!(raw_linux_syscall(&mut emu, NR64_FORK, [0; 3]), 0);
    let child = emu.processes.pid;
    assert_eq!(emu.processes.ppid, parent);
    assert!(emu.maps.write_dword(DATA, 1));

    raw_linux_syscall(&mut emu, NR64_EXIT_GROUP, [CHILD_STATUS, 0, 0]);
    assert_eq!(emu.processes.pid, parent);
    assert_eq!(emu.pc(), CODE + PREFIXED_SYSCALL.len() as u64);
    assert_eq!(emu.regs().rax, child);
    assert_eq!(emu.maps.read_dword(DATA), Some(0));

    assert_eq!(
        raw_linux_syscall(&mut emu, NR64_WAIT4, [child, u64::MAX, 0]),
        EFAULT
    );
    assert_eq!(
        raw_linux_syscall(&mut emu, NR64_WAIT4, [child, STATUS, 0]),
        child
    );
    assert_eq!(
        emu.maps.read_dword(STATUS),
        Some((CHILD_STATUS << 8) as u32)
    );
    assert_eq!(
        raw_linux_syscall(&mut emu, NR64_WAIT4, [child, STATUS, 0]),
        ECHILD
    );
}

#[test]
fn i386_parent_reaps_child_that_execs_an_amd64_image() {
    use crate::syscall::linux::constants::syscalls::{
        NR32_EXECVE, NR32_EXIT, NR32_FORK, NR32_WAITPID, NR64_EXIT,
    };
    use crate::threading::process::exit_status;
    use byteorder::{LittleEndian as LE, WriteBytesExt};
    use goblin::elf::{
        header::{ELFCLASS64, ELFDATA2LSB, ELFMAG, EM_X86_64, ET_EXEC},
        program_header::{PF_R, PF_X, PT_LOAD},
    };

    const CHILD_STATUS: u8 = 37;
    const CHILD_PATH: &str = "/bin/child64";
    const CHILD_BASE: u64 = 0x0040_0000;
    const EHDR: u16 = 64;
    const PHDR: u16 = 56;
    const PARENT_BASE: u64 = 0x0804_8000;
    const PATH: u64 = 0x0805_0000;
    const STATUS: u64 = PATH + 64;

    #[rustfmt::skip]
    const CHILD_CODE: &[u8] = &[
        0x41, 0xb8, CHILD_STATUS, 0, 0, 0,   // mov r8d, CHILD_STATUS
        0xb8, NR64_EXIT as u8, 0, 0, 0,      // mov eax, SYS_exit
        0x44, 0x89, 0xc7,                    // mov edi, r8d
        0x0f, 0x05,                          // syscall
    ];
    let entry_offset = (EHDR + PHDR) as u64;
    let size = entry_offset + CHILD_CODE.len() as u64;
    let mut image = ELFMAG.to_vec();
    image.extend_from_slice(&[ELFCLASS64, ELFDATA2LSB, 1]);
    image.resize(16, 0);
    image.write_u16::<LE>(ET_EXEC).unwrap();
    image.write_u16::<LE>(EM_X86_64).unwrap();
    image.write_u32::<LE>(1).unwrap();
    for value in [CHILD_BASE + entry_offset, EHDR as u64, 0] {
        image.write_u64::<LE>(value).unwrap();
    }
    image.write_u32::<LE>(0).unwrap();
    for value in [EHDR, PHDR, 1, 0, 0, 0] {
        image.write_u16::<LE>(value).unwrap();
    }
    image.write_u32::<LE>(PT_LOAD).unwrap();
    image.write_u32::<LE>(PF_R | PF_X).unwrap();
    for value in [0, CHILD_BASE, CHILD_BASE, size, size, 4096] {
        image.write_u64::<LE>(value).unwrap();
    }
    image.extend_from_slice(CHILD_CODE);

    #[rustfmt::skip]
    const PARENT_CODE: &[u8] = &[
        0xb8, NR32_FORK as u8, 0, 0, 0,      // mov eax, SYS_fork
        0xcd, 0x80,                          // int 80h
        0x85, 0xc0,                          // test eax, eax
        0x75, 6,                             // jnz parent
        0xb0, NR32_EXECVE as u8,             // child: mov al, SYS_execve
        0xcd, 0x80,                          // execve(ebx=PATH, NULL, NULL)
        0x0f, 0x0b,                          // ud2: execve returned
        0x89, 0xc3,                          // parent: mov ebx, eax (child pid)
        0x89, 0xf1,                          // mov ecx, esi (status address)
        0xb8, NR32_WAITPID as u8, 0, 0, 0,   // mov eax, SYS_waitpid
        0xcd, 0x80,                          // waitpid(child, STATUS, 0)
        0x8b, 0x19,                          // mov ebx, [ecx]
        0xc1, 0xeb, 8,                       // shr ebx, 8 (child exit code)
        0xb8, NR32_EXIT as u8, 0, 0, 0,      // mov eax, SYS_exit
        0xcd, 0x80,                          // exit(child exit code)
    ];
    let mut emu = emu32();
    emu.os = OperatingSystem::Linux;
    emu.cfg.max_instructions = Some(100);
    emu.set_linux_files(Vec::new(), Vec::new());
    emu.linux_files
        .as_mut()
        .unwrap()
        .mount_executable(CHILD_PATH, &image)
        .unwrap();
    emu.maps
        .create_map(
            "code",
            PARENT_BASE,
            PARENT_CODE.len() as u64,
            Permission::READ_EXECUTE,
        )
        .unwrap()
        .force_write_bytes(PARENT_BASE, PARENT_CODE);
    emu.maps
        .create_map("data", PATH, 4096, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_string(PATH, CHILD_PATH);
    let regs = emu.regs_mut();
    [regs.rbx, regs.rcx, regs.rdx, regs.rsi] = [PATH, 0, 0, STATUS];
    emu.set_pc(PARENT_BASE);
    let parent = emu.processes.pid;

    emu.run(None).unwrap();

    assert_eq!(emu.fault_count, 0);
    assert_eq!(emu.cfg.arch, Arch::X86);
    assert_eq!(emu.processes.pid, parent);
    assert_eq!(
        emu.maps.read_dword(STATUS),
        Some((CHILD_STATUS as u32) << 8)
    );
    assert_eq!(
        emu.processes.exit_status,
        Some(exit_status(CHILD_STATUS as u64))
    );
}

#[test]
fn i386_waitpid_resumes_a_sleeping_child_before_reaping_it() {
    use crate::syscall::linux::constants::errors::{ECHILD, EFAULT, EINVAL};
    use crate::syscall::linux::constants::syscalls::{
        NR32_EXIT, NR32_FORK, NR32_NANOSLEEP, NR32_WAITPID,
    };

    const CODE: u64 = 0x400000;
    const REQUEST: u64 = 0x500000;
    const STATUS: u64 = REQUEST + 8;
    const WNOHANG: u64 = 1;
    const CHILD_STATUS: u64 = 7;

    helpers::setup();
    let mut emu = emu32();
    emu.os = OperatingSystem::Linux;
    emu.maps
        .create_map("syscalls", CODE, 4, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    assert!(emu.maps.write_bytes(CODE, &[0xcd, 0x80, 0xcd, 0x80]));
    emu.maps
        .create_map("timespec", REQUEST, 12, Permission::READ_WRITE)
        .unwrap();
    emu.set_pc(CODE);
    let parent = emu.processes.pid;
    assert_eq!(
        raw_linux_syscall(&mut emu, NR32_NANOSLEEP, [0, 0, 0]),
        EFAULT
    );
    assert!(emu.maps.write_qword(REQUEST, u32::MAX as u64));
    assert_eq!(
        raw_linux_syscall(&mut emu, NR32_NANOSLEEP, [REQUEST, 0, 0]),
        EINVAL
    );
    assert!(emu.maps.write_qword(REQUEST, 60));

    assert_eq!(raw_linux_syscall(&mut emu, NR32_FORK, [0; 3]), 0);
    let child = emu.processes.pid;
    assert_ne!(parent, child);
    emu.set_pc(CODE + 2);
    assert_eq!(
        raw_linux_syscall(&mut emu, NR32_NANOSLEEP, [REQUEST, 0, 0]),
        child
    );
    assert_eq!((emu.processes.pid, emu.pc()), (parent, CODE + 2));
    assert_eq!(
        raw_linux_syscall(&mut emu, NR32_WAITPID, [child, STATUS, WNOHANG]),
        0
    );
    assert_eq!(emu.maps.read_dword(STATUS), Some(0));

    raw_linux_syscall(&mut emu, NR32_WAITPID, [child, STATUS, 0]);
    assert_eq!((emu.processes.pid, emu.pc()), (child, CODE + 4));
    assert_eq!(emu.regs().get_eax(), 0);
    raw_linux_syscall(&mut emu, NR32_EXIT, [CHILD_STATUS, 0, 0]);
    assert_eq!((emu.processes.pid, emu.pc()), (parent, CODE + 2));
    assert_eq!(emu.regs().get_eax(), NR32_WAITPID);
    syscall32::gateway(&mut emu);
    assert_eq!(emu.regs().get_eax(), child);
    assert_eq!(
        emu.maps.read_dword(STATUS),
        Some((CHILD_STATUS << 8) as u32)
    );
    assert_eq!(
        raw_linux_syscall(&mut emu, NR32_WAITPID, [child, STATUS, 0]),
        ECHILD
    );
}

#[test]
fn raw_linux_clocks_validate_the_full_timespec_on_each_abi() {
    use crate::syscall::linux::constants::errors::{EFAULT, EINVAL};
    use crate::syscall::linux::constants::syscalls::*;
    const DATA: u64 = 0x500000;
    const SIZE: u64 = 4096;
    helpers::setup();
    for (arch, number, width) in [
        (Arch::X86, NR32_CLOCK_GETTIME, 8),
        (Arch::X86, NR32_CLOCK_GETTIME64, 16),
        (Arch::X86_64, NR64_CLOCK_GETTIME, 16),
        (Arch::Aarch64, NRA64_CLOCK_GETTIME, 16),
    ] {
        let mut emu = match arch {
            Arch::X86 => emu32(),
            Arch::X86_64 => emu64(),
            Arch::Aarch64 => emu_aarch64(),
        };
        emu.os = OperatingSystem::Linux;
        emu.maps
            .create_map("clock.test", DATA, SIZE, Permission::READ_WRITE)
            .unwrap();
        for (id, ptr, status) in [
            (1, DATA, 0),
            (u32::MAX as u64, DATA, EINVAL),
            (1, DATA + SIZE - 4, EFAULT),
            (1, 0, EFAULT),
        ] {
            emu.maps.fill_range(DATA, 0xcc, SIZE as usize);
            let result = raw_linux_syscall(&mut emu, number, [id, ptr, 0]);
            assert_eq!(result, status, "{arch:?} {number}");
            if status == 0 {
                let nanos = if width == 8 {
                    emu.maps.read_dword(DATA + 4).map(u64::from)
                } else {
                    emu.maps.read_qword(DATA + 8)
                };
                assert!(nanos.is_some_and(|n| n < 1_000_000_000));
                assert_eq!(emu.maps.read_byte(DATA + width), Some(0xcc));
            } else {
                assert_eq!(emu.maps.read_dword(DATA), Some(0xcccc_cccc));
                assert_eq!(emu.maps.read_dword(DATA + SIZE - 4), Some(0xcccc_cccc));
            }
        }
    }
}

#[test]
fn linux_system_calls_write_only_the_requested_guest_buffer() {
    use crate::syscall::linux::constants::errors::{EFAULT, EINVAL};
    const BASE: u64 = 0x400000;
    const SIZE: usize = 390;
    for (mut emu, uname, random, machine) in [
        (emu32(), 122, 355, "i686"),
        (emu64(), 63, 318, "x86_64"),
        (emu_aarch64(), 160, 278, "aarch64"),
    ] {
        emu.os = OperatingSystem::Linux;
        emu.maps
            .create_map("utsname", BASE, SIZE as u64 + 1, Permission::READ_WRITE)
            .unwrap();
        emu.maps.fill_range(BASE, 0xcc, SIZE + 1);
        for invalid in [0, BASE + 2] {
            assert_eq!(raw_linux_syscall(&mut emu, uname, [invalid, 0, 0]), EFAULT);
        }
        assert_eq!(emu.maps.read_byte(BASE + 2), Some(0xcc));
        assert_eq!(raw_linux_syscall(&mut emu, uname, [BASE, 0, 0]), 0);
        assert_eq!(emu.maps.read_string(BASE), "Linux");
        assert_eq!(emu.maps.read_string(BASE + 4 * 65), machine);
        assert_eq!(emu.maps.read_byte(BASE + SIZE as u64), Some(0xcc));
        for (addr, size, flags, expected) in [
            (0, 0, 0, 0),
            (0, 1, 0, EFAULT),
            (BASE, 9, 6, EINVAL),
            (BASE, 9, 1, 9),
        ] {
            assert_eq!(
                raw_linux_syscall(&mut emu, random, [addr, size, flags]),
                expected
            );
        }
        assert_ne!(emu.maps.read_qword(BASE), Some(0));
        assert_eq!(emu.maps.read_byte(BASE + 9), Some(0));
    }
}

#[test]
fn linux_credential_queries_use_the_guest_id_and_native_element_width() {
    use crate::syscall::linux::GUEST_ID;
    use crate::syscall::linux::constants::errors::{EFAULT, EINVAL};
    use crate::syscall::linux::constants::syscalls::*;
    const DATA: u64 = 0x500000;
    for (mut emu, uid, groups, res, width) in [
        (emu32(), NR32_GETUID, NR32_GETGROUPS, NR32_GETRESUID, 2usize),
        (
            emu32(),
            NR32_GETUID32,
            NR32_GETGROUPS32,
            NR32_GETRESGID32,
            4,
        ),
        (emu64(), NR64_GETUID, NR64_GETGROUPS, NR64_GETRESUID, 4),
        (
            emu_aarch64(),
            NRA64_GETEGID,
            NRA64_GETGROUPS,
            NRA64_GETRESGID,
            4,
        ),
    ] {
        emu.maps
            .create_map("ids", DATA, 0x1000, Permission::READ_WRITE)
            .unwrap();
        assert_eq!(
            raw_linux_syscall(&mut emu, uid, [0; 3]),
            u64::from(GUEST_ID)
        );
        assert_eq!(
            raw_linux_syscall(&mut emu, groups, [u64::MAX, 0, 0]) as i32,
            EINVAL as i32
        );
        assert_eq!(raw_linux_syscall(&mut emu, groups, [0, 0, 0]), 1);
        assert_eq!(
            raw_linux_syscall(&mut emu, res, [0, DATA, DATA]) as i32,
            EFAULT as i32
        );
        emu.maps.write_bytes(DATA, &[0xa5; 16]);
        assert_eq!(
            raw_linux_syscall(&mut emu, res, [DATA, DATA + 4, DATA + 8]),
            0
        );
        for offset in [0, 4, 8] {
            assert_eq!(emu.maps.read_word(DATA + offset), Some(GUEST_ID as u16));
        }
        assert_eq!(emu.maps.read_byte(DATA + 8 + width as u64), Some(0xa5));
    }
}

#[test]
fn i386_socketcall_reads_the_complete_argument_array_from_ecx() {
    use crate::syscall::linux::constants::errors::{EFAULT, EINVAL};
    use crate::syscall::linux::constants::syscalls::{NR32_SOCKETCALL, SYS_RECVFROM, SYS_SOCKET};
    use crate::winapi::helper::socket_close;

    let mut emu = emu32();
    let args = 0x20000;
    emu.maps
        .create_map("socketcall arguments", args, 24, Permission::READ_WRITE)
        .unwrap();
    emu.regs_mut().set_esp(0x40000);
    let invoke = |emu: &mut Emu, selector: u32, address: u64| {
        emu.regs_mut().set_eax(NR32_SOCKETCALL);
        emu.regs_mut().set_ebx(selector as u64);
        emu.regs_mut().set_ecx(address);
        syscall32::gateway(emu);
        emu.regs().get_eax()
    };
    assert_eq!(
        invoke(&mut emu, SYS_SOCKET, args + 16),
        EFAULT as u32 as u64
    );
    assert_eq!(invoke(&mut emu, 0, args), EINVAL as u32 as u64);
    assert!(emu.maps.write_dword(args, 2));
    assert!(emu.maps.write_dword(args + 4, 2));
    let descriptor = invoke(&mut emu, SYS_SOCKET, args);
    assert!(emu.maps.write_dword(args, descriptor as u32));
    assert!(emu.maps.write_dword(args + 8, 31));
    assert_eq!(invoke(&mut emu, SYS_RECVFROM, args), 31);
    assert_eq!(emu.regs().get_esp(), 0x40000);
    assert!(socket_close(descriptor));
}

#[test]
fn long_mode_int80_uses_i386_numbers_arguments_and_buffer_layout() {
    const CODE: u64 = 0x1_0040_0000;
    const BUFFER: u64 = 0x500000;
    const HIGH_BITS: u64 = 0xabcd_1234_0000_0000;
    const I386_WRITEV: u64 = 146;
    const I386_SOCKETCALL: u64 = 102;
    let mut emu = Emu::new(Arch::X86_64);
    emu.os = OperatingSystem::Linux;
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.disable_ctrlc();
    for (name, base) in [("int80.code", CODE), ("int80.data", BUFFER)] {
        emu.maps
            .create_map(name, base, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
    }
    assert!(emu.maps.write_bytes(CODE, &[0xcd, 0x80]));
    assert!(emu.maps.write_dword(BUFFER, (BUFFER + 8) as u32));
    assert!(emu.maps.write_dword(BUFFER + 4, 2));
    assert!(emu.maps.write_bytes(BUFFER + 8, b"ok"));
    let regs = emu.regs_mut();
    regs.rbx = HIGH_BITS | 1;
    regs.rcx = HIGH_BITS | BUFFER;
    regs.rdx = HIGH_BITS | 1;
    regs.rsi = HIGH_BITS;
    regs.rsp = CODE + 0x800;
    for (number, expected) in [(I386_WRITEV, 2), (I386_SOCKETCALL, ENOSYS)] {
        emu.set_pc(CODE);
        emu.regs_mut().rax = HIGH_BITS | number;
        assert!(emu.step());
        assert_eq!(emu.regs().rax, expected);
        assert_eq!(emu.pc(), CODE + 2);
        assert_eq!(emu.sp(), CODE + 0x800);
        assert_eq!(emu.regs().rbx, HIGH_BITS | 1);
        assert_eq!(emu.regs().rcx, HIGH_BITS | BUFFER);
        assert_eq!(emu.regs().rsi, HIGH_BITS);
    }
    assert_eq!(emu.emulated_stdout, b"ok");
}
