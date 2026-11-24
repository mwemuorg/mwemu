// Use explicit ABI numbers because Linux syscall tables have gaps.
macro_rules! syscall_table {
    ($lookup:ident; $($constant:ident = $number:literal => $name:literal),+ $(,)?) => {
        $(pub const $constant: u64 = $number;)+

        pub(crate) fn $lookup(number: u64) -> Option<&'static str> {
            match number {
                $($constant => Some($name),)+
                _ => None,
            }
        }
    };
}

mod i386;
mod x86_64;

pub use i386::*;
pub use x86_64::*;

pub const SYS_SOCKET: u32 = 1;
pub const SYS_BIND: u32 = 2;
pub const SYS_CONNECT: u32 = 3;
pub const SYS_LISTEN: u32 = 4;
pub const SYS_ACCEPT: u32 = 5;
pub const SYS_GETSOCKNAME: u32 = 6;
pub const SYS_GETPEERNAME: u32 = 7;
pub const SYS_SOCKETPAIR: u32 = 8;
pub const SYS_SEND: u32 = 9;
pub const SYS_RECV: u32 = 10;
pub const SYS_SENDTO: u32 = 11;
pub const SYS_RECVFROM: u32 = 12;
pub const SYS_SHUTDOWN: u32 = 13;
pub const SYS_SETSOCKOPT: u32 = 14;
pub const SYS_GETSOCKOPT: u32 = 15;
pub const SYS_SENDMSG: u32 = 16;
pub const SYS_RECVMSG: u32 = 17;
pub const SYS_ACCEPT4: u32 = 18;
pub const SYS_RECVMMSG: u32 = 19;
pub const SYS_SENDMMSG: u32 = 20;

pub const NRA64_GETCWD: u64 = 17;
pub const NRA64_EVENTFD2: u64 = 19;
pub const NRA64_DUP: u64 = 23;
pub const NRA64_DUP3: u64 = 24;
pub const NRA64_FCNTL: u64 = 25;
pub const NRA64_IOCTL: u64 = 29;
pub const NRA64_MKDIRAT: u64 = 34;
pub const NRA64_UNLINKAT: u64 = 35;
pub const NRA64_SYMLINKAT: u64 = 36;
pub const NRA64_FTRUNCATE: u64 = 46;
pub const NRA64_FACCESSAT: u64 = 48;
pub const NRA64_CHDIR: u64 = 49;
pub const NRA64_FCHMOD: u64 = 52;
pub const NRA64_OPENAT: u64 = 56;
pub const NRA64_CLOSE: u64 = 57;
pub const NRA64_PIPE2: u64 = 59;
pub const NRA64_GETDENTS64: u64 = 61;
pub const NRA64_LSEEK: u64 = 62;
pub const NRA64_READ: u64 = 63;
pub const NRA64_WRITE: u64 = 64;
pub const NRA64_READV: u64 = 65;
pub const NRA64_WRITEV: u64 = 66;
pub const NRA64_PREAD64: u64 = 67;
pub const NRA64_READLINKAT: u64 = 78;
pub const NRA64_NEWFSTATAT: u64 = 79;
pub const NRA64_FSTAT: u64 = 80;
pub const NRA64_EXIT: u64 = 93;
pub const NRA64_EXIT_GROUP: u64 = 94;
pub const NRA64_SET_TID_ADDRESS: u64 = 96;
pub const NRA64_FUTEX: u64 = 98;
pub const NRA64_CLOCK_GETTIME: u64 = 113;
pub const NRA64_RT_SIGACTION: u64 = 134;
pub const NRA64_RT_SIGPROCMASK: u64 = 135;
pub const NRA64_GETRESUID: u64 = 148;
pub const NRA64_GETRESGID: u64 = 150;
pub const NRA64_GETGROUPS: u64 = 158;
pub const NRA64_UNAME: u64 = 160;
pub const NRA64_GETTIMEOFDAY: u64 = 169;
pub const NRA64_GETPID: u64 = 172;
pub const NRA64_GETPPID: u64 = 173;
pub const NRA64_GETUID: u64 = 174;
pub const NRA64_GETEUID: u64 = 175;
pub const NRA64_GETGID: u64 = 176;
pub const NRA64_GETEGID: u64 = 177;
pub const NRA64_GETTID: u64 = 178;
pub const NRA64_BRK: u64 = 214;
pub const NRA64_MUNMAP: u64 = 215;
pub const NRA64_EXECVE: u64 = 221;
pub const NRA64_MMAP: u64 = 222;
pub const NRA64_MPROTECT: u64 = 226;
pub const NRA64_MSYNC: u64 = 227;
pub const NRA64_GETRANDOM: u64 = 278;
pub const NRA64_MEMFD_CREATE: u64 = 279;
pub const NRA64_EXECVEAT: u64 = 281;
pub const NRA64_STATX: u64 = 291;
