pub const MAX_PATH: usize = 260;

/// NT device of the guest's only volume (C:), shared by every API that
/// turns a DOS path into an NT device path.
pub const SYSTEM_VOLUME_DEVICE: &str = "\\Device\\HarddiskVolume3";

pub const FILE_SHARE_READ: u32 = 1;
pub const FILE_SHARE_WRITE: u32 = 2;

pub const FILE_BEGIN: u32 = 0;
pub const FILE_CURRENT: u32 = 1;
pub const FILE_END: u32 = 2;

pub const INVALID_FILE_ATTRIBUTES: u32 = 0xffff_ffff;
pub const INVALID_FILE_SIZE: u64 = 0xffff_ffff;
pub const INVALID_SET_FILE_POINTER: u64 = 0xffff_ffff;

pub const GET_FILEEX_INFO_STANDARD: u32 = 0;
pub const FILE_END_OF_FILE_INFO_CLASS: u32 = 6;
pub const FILE_ATTRIBUTE_TAG_INFO_CLASS: u32 = 9;

pub const FILE_NAME_OPENED: u32 = 8;

pub const VOLUME_NAME_GUID: u32 = 1;
pub const VOLUME_NAME_NT: u32 = 2;
pub const VOLUME_NAME_NONE: u32 = 4;

pub const FILE_MAP_COPY: u32 = 1;
pub const FILE_MAP_WRITE: u32 = 2;
pub const FILE_MAP_READ: u32 = 4;
pub const FILE_MAP_EXECUTE: u32 = 0x20;
pub const FILE_MAP_ALL_ACCESS: u32 = 0xf001f;

pub const FILE_READ_DATA: u32 = 1;
pub const FILE_WRITE_DATA: u32 = 2;
pub const FILE_APPEND_DATA: u32 = 4;
pub const FILE_WRITE_ATTRIBUTES: u32 = 0x0100;

pub const CREATE_NEW: usize = 1;
pub const CREATE_ALWAYS: usize = 2;
pub const OPEN_EXISTING: usize = 3;
pub const OPEN_ALWAYS: usize = 4;
pub const TRUNCATE_EXISTING: usize = 5;

pub const FILE_ATTRIBUTE_READONLY: usize = 0x1;
pub const FILE_ATTRIBUTE_HIDDEN: usize = 0x2;
pub const FILE_ATTRIBUTE_SYSTEM: usize = 0x4;
pub const FILE_ATTRIBUTE_DIRECTORY: usize = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: usize = 0x20;
pub const FILE_ATTRIBUTE_NORMAL: usize = 0x80;
pub const FILE_ATTRIBUTE_TEMPORARY: usize = 0x100;
pub const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x0200;
pub const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
pub const FILE_ATTRIBUTE_COMPRESSED: usize = 0x800;
pub const FILE_ATTRIBUTE_OFFLINE: usize = 0x1000;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: usize = 0x2000;
pub const FILE_ATTRIBUTE_ENCRYPTED: usize = 0x4000;

pub const FILE_FLAG_WRITE_THROUGH: usize = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: usize = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: usize = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: usize = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: usize = 0x8000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: usize = 0x4000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: usize = 0x2000000;
pub const FILE_FLAG_POSIX_SEMANTICS: usize = 0x1000000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: usize = 0x200000;
pub const FILE_FLAG_OPEN_NO_RECALL: usize = 0x100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: usize = 0x80000;

pub const MOVEFILE_REPLACE_EXISTING: u32 = 1;
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1;

pub const FILE_TYPE_UNKNOWN: u64 = 0;
pub const FILE_TYPE_DISK: u64 = 1;
pub const FILE_TYPE_CHAR: u64 = 2;
pub const FILE_TYPE_PIPE: u64 = 3;

pub const STD_INPUT_HANDLE: u32 = 0xffff_fff6;
pub const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;
pub const STD_ERROR_HANDLE: u32 = 0xffff_fff4;

pub const RT_STRING: u32 = 6;
pub const RT_VERSION: u32 = 16;

pub const PIPE_ACCESS_INBOUND: u32 = 1;
pub const PIPE_ACCESS_OUTBOUND: u32 = 2;
pub const PIPE_ACCESS_DUPLEX: u32 = PIPE_ACCESS_INBOUND | PIPE_ACCESS_OUTBOUND;

pub const MAILSLOT_NO_MESSAGE: u32 = u32::MAX;

pub const PATHCCH_MAX_CCH: usize = 0x8000;
