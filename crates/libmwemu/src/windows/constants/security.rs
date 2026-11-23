pub const READ_CONTROL: u32 = 0x0002_0000;
pub const SYNCHRONIZE: u32 = 0x0010_0000;

pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;
pub const GENERIC_EXECUTE: u32 = 0x20000000;
pub const GENERIC_ALL: u32 = 0x10000000;

pub const KEY_QUERY_VALUE: u32 = 0x0001;
pub const KEY_WOW64_64KEY: u32 = 0x0100;
pub const KEY_WOW64_32KEY: u32 = 0x0200;
pub const KEY_READ: u32 = 0x0002_0019;
pub const KEY_ALL_ACCESS: u32 = 0x000f_003f;

pub const ACL_REVISION: u32 = 2;
pub const ACL_REVISION_DS: u32 = 4;
pub const ACL_REVISION_INFORMATION: u32 = 1;
pub const ACL_SIZE_INFORMATION: u32 = 2;

pub const RRF_RT_REG_DWORD: u32 = 0x10;
pub const RRF_RT_DWORD: u32 = 0x18;
pub const RRF_RT_QWORD: u32 = 0x48;
pub const RRF_RT_ANY: u32 = 0xffff;
pub const RRF_WOW64_MASK: u32 = 0x0003_0000;
pub const RRF_NOEXPAND: u32 = 0x1000_0000;
pub const RRF_ZEROONFAILURE: u32 = 0x2000_0000;

pub const TOKEN_QUERY: u32 = 0x0008;
pub const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0020;
pub const CURRENT_PROCESS_TOKEN: i32 = -4;
pub const TOKEN_USER: u32 = 1;
pub const MAXIMUM_ALLOWED: u32 = 0x0200_0000;

pub const SECURITY_DESCRIPTOR_REVISION: u8 = 1;
pub const SE_OWNER_DEFAULTED: u16 = 1;
pub const SE_GROUP_DEFAULTED: u16 = 2;
pub const SE_DACL_PRESENT: u16 = 4;
pub const SE_DACL_DEFAULTED: u16 = 8;
pub const SE_SACL_PRESENT: u16 = 0x10;
pub const SE_SACL_DEFAULTED: u16 = 0x20;
pub const SE_SELF_RELATIVE: u16 = 0x8000;

pub const ACCESS_ALLOWED_ACE_TYPE: u8 = 0;
pub const ACCESS_DENIED_ACE_TYPE: u8 = 1;

pub const WIN_NULL_SID: u32 = 0;
pub const WIN_WORLD_SID: u32 = 1;
pub const WIN_LOCAL_SID: u32 = 2;
pub const WIN_LOCAL_SYSTEM_SID: u32 = 22;
pub const WIN_LOCAL_SERVICE_SID: u32 = 23;
pub const WIN_NETWORK_SERVICE_SID: u32 = 24;
pub const WIN_BUILTIN_DOMAIN_SID: u32 = 25;
pub const WIN_BUILTIN_ADMINISTRATORS_SID: u32 = 26;
pub const WIN_BUILTIN_USERS_SID: u32 = 27;
pub const WIN_BUILTIN_GUESTS_SID: u32 = 28;

pub const SID_TYPE_USER: u32 = 1;
