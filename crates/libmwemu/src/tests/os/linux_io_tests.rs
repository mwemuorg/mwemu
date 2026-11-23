use crate::api::linux::libc;
use crate::api::unix;
use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{
    EAGAIN, EBADF, EFAULT, EINVAL, ENOENT, ENOSYS, EPIPE,
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
    Close,
}

impl Operation {
    fn number(self, arch: Arch) -> u64 {
        let numbers = match self {
            Self::Read => [3, 0, 63],
            Self::Write => [4, 1, 64],
            Self::Writev => [146, 20, 66],
            Self::Fcntl => [55, 72, 25],
            Self::Pipe2 => [331, 293, 59],
            Self::Eventfd2 => [328, 290, 19],
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
    let number = operation.number(emu.cfg.arch);
    match emu.cfg.arch {
        Arch::X86 => {
            let regs = emu.regs_mut();
            regs.rax = number;
            [regs.rbx, regs.rcx, regs.rdx] = args;
            syscall32::gateway(emu);
            emu.regs().get_eax() as i32 as i64
        }
        Arch::X86_64 => {
            let regs = emu.regs_mut();
            regs.rax = number;
            [regs.rdi, regs.rsi, regs.rdx] = args;
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
    }
}
