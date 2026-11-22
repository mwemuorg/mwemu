pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;

pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_SETFL: u32 = 4;

pub const O_RDONLY: u64 = 0;
pub const O_WRONLY: u64 = 1;

pub const O_RDWR: u64 = 2;
pub const O_NONBLOCK: u32 = 0x800;
pub const O_CLOEXEC: u32 = 0x80000;

pub const FD_CLOEXEC: u64 = 1;

pub const EFD_SEMAPHORE: u32 = 1;
pub const EFD_NONBLOCK: u32 = O_NONBLOCK;
pub const EFD_CLOEXEC: u32 = O_CLOEXEC;

pub const PIPE_BUF: usize = 4096;

pub const UIO_MAXIOV: u32 = 1024;
