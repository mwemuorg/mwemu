//! Windows C runtime constants.
pub const ENOENT: i32 = 2;
pub const EIO: i32 = 5;
pub const EBADF: i32 = 9;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EEXIST: i32 = 17;
pub const EINVAL: i32 = 22;
pub const EMFILE: i32 = 24;
pub const ERANGE: i32 = 34;
pub const ENOSYS: i32 = 40;

pub const STRUNCATE: i32 = 80;
pub const _TRUNCATE: u64 = u64::MAX;

pub const CRT_ARGV_NO_ARGUMENTS: i32 = 0;
pub const CRT_ARGV_UNEXPANDED_ARGUMENTS: i32 = 1;
pub const CRT_ARGV_EXPANDED_ARGUMENTS: i32 = 2;

pub const _ENABLE_PER_THREAD_LOCALE: i32 = 1;
pub const _DISABLE_PER_THREAD_LOCALE: i32 = 2;

pub const EOF: u64 = 0xffff_ffff;
pub const RAND_MAX: u32 = 0x7fff;

pub const SEEK_SET: u64 = 0;
pub const SEEK_CUR: u64 = 1;
pub const SEEK_END: u64 = 2;

pub const _O_RDONLY: i32 = 0x0000;
pub const _O_WRONLY: i32 = 0x0001;
pub const _O_RDWR: i32 = 0x0002;
pub const _O_APPEND: i32 = 0x0008;
pub const _O_RANDOM: i32 = 0x0010;
pub const _O_SEQUENTIAL: i32 = 0x0020;
pub const _O_TEMPORARY: i32 = 0x0040;
pub const _O_NOINHERIT: i32 = 0x0080;
pub const _O_CREAT: i32 = 0x0100;
pub const _O_TRUNC: i32 = 0x0200;
pub const _O_EXCL: i32 = 0x0400;
pub const _O_SHORT_LIVED: i32 = 0x1000;
pub const _O_OBTAIN_DIR: i32 = 0x2000;
pub const _O_TEXT: i32 = 0x4000;
pub const _O_BINARY: i32 = 0x8000;
pub const _O_WTEXT: i32 = 0x10000;
pub const _O_U16TEXT: i32 = 0x20000;
pub const _O_U8TEXT: i32 = 0x40000;
pub const _O_RAW: i32 = _O_BINARY;

pub const _S_IREAD: i32 = 0x0100;
pub const _S_IWRITE: i32 = 0x0080;
