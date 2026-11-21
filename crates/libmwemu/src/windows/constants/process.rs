//
pub const CURRENT_PROCESS_HANDLE: u64 = 1975;
// NtCurrentProcess() and NtCurrentThread(); narrowed at the x86 ABI boundary.
pub const NT_CURRENT_PROCESS: u64 = 0xffff_ffff_ffff_ffff;
pub const NT_CURRENT_THREAD: u64 = 0xffff_ffff_ffff_fffe;
pub const WAIT_OBJECT_0: u64 = 0;
pub const WAIT_TIMEOUT: u64 = 0x00000102;
pub const WAIT_FAILED: u64 = 0xFFFFFFFF;
pub const CREATE_SUSPENDED: u64 = 0x00000004;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: u64 = 0x0001_0000;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4;
pub const INVALID_HANDLE_VALUE_64: u64 = 0xFFFFFFFFFFFFFFFF;
pub const INVALID_HANDLE_VALUE_32: u64 = 0xFFFFFFFF;
pub const RETURN_THREAD: u32 = 0x11223344;
pub const INVALID_HANDLE_VALUE: usize = !0;
// TLS Callback Reason:
pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const DLL_PROCESS_DETACH: u32 = 0;
pub const DLL_THREAD_ATTACH: u32 = 2;
pub const DLL_THREAD_DETACH: u32 = 3;
/// `PROCESSINFOCLASS` for `NtQueryInformationProcess` (`ProcessBasicInformation` = 0).
pub const PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION: u64 = 0;
/// `ProcessCookie` (see Windows SDK `PROCESSINFOCLASS`).
pub const PROCESS_INFORMATION_CLASS_PROCESS_COOKIE: u64 = 0x24;
/// `ProcessDebugPort` — query returns 0 when no debugger (see Windows SDK).
pub const PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_PORT: u64 = 7;
/// `THREADINFOCLASS` (`ThreadBasicInformation` = 0).
pub const THREAD_INFORMATION_CLASS_THREAD_BASIC_INFORMATION: u64 = 0;
pub const THREAD_INFORMATION_CLASS_THREAD_QUERY_SET_WIN32_START_ADDRESS: u64 = 9;
