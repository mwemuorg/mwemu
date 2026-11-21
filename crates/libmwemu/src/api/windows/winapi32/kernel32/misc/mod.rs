pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    AddAtomA, AddAtomW, CryptCreateHash, DuplicateHandle, FindAtomA, FindAtomW, FormatMessageA,
    FormatMessageW, GetAtomNameA, GetAtomNameW, GetLastError, GetStartupInfoA, GetStartupInfoW,
    GlobalAddAtomA, GlobalAddAtomExA, GlobalAddAtomExW, GlobalAddAtomW, GlobalFindAtomA,
    GlobalFindAtomW, GlobalFlags, GlobalGetAtomNameA, GlobalGetAtomNameW, GlobalLock, GlobalUnlock,
    IsWow64Process, OutputDebugStringA, OutputDebugStringW, SetHandleCount, SetLastError, Sleep,
};

pub use crate::api::windows::libraries::kernel32::misc::*;
use crate::winapi::common::fallback::unsupported_exports;

use crate::winapi::common::files::file_entry_points;

file_entry_points!(
    "kernel32",
    CheckNameLegalDOS8Dot3A,
    CheckNameLegalDOS8Dot3W,
    CreateHardLinkTransactedA,
    CreateHardLinkTransactedW,
    CreateSymbolicLinkA,
    CreateSymbolicLinkTransactedA,
    CreateSymbolicLinkTransactedW,
    CreateSymbolicLinkW,
    GetExpandedNameA,
    GetExpandedNameW,
    LZClose,
    LZCopy,
    LZInit,
    LZRead,
    LZSeek,
    LZStart,
);

mod query_performance_counter;
pub use query_performance_counter::QueryPerformanceCounter;

mod win_exec;
pub use win_exec::WinExec;

mod global_re_alloc;
pub use global_re_alloc::GlobalReAlloc;

mod local_size;
pub use local_size::LocalSize;

pub use crate::winapi::libraries::kernel32::misc::MulDiv;

mod get_tick_count64;
pub use get_tick_count64::GetTickCount64;

mod query_performance_frequency;
pub use query_performance_frequency::QueryPerformanceFrequency;

unsupported_exports!(
    "kernel32",
    AddIntegrityLabelToBoundaryDescriptor,
    AddLocalAlternateComputerNameA,
    AddLocalAlternateComputerNameW,
    BindIoCompletionCallback,
    CheckElevation,
    ConvertNLSDayOfWeekToWin32DayOfWeek,
    CreateBoundaryDescriptorA,
    CreateBoundaryDescriptorW,
    DnsHostnameToComputerNameA,
    DnsHostnameToComputerNameW,
    EnumerateLocalComputerNamesA,
    EnumerateLocalComputerNamesW,
    FreeMemoryJobObject,
    GetApplicationRestartSettingsWorker,
    GetComPlusPackageInstallStatus,
    GetDevicePowerState,
    GetDurationFormat,
    GetNumaAvailableMemoryNode,
    GetNumaAvailableMemoryNodeEx,
    GetNumaProximityNode,
    GlobalFix,
    GlobalMemoryStatus,
    GlobalUnWire,
    GlobalUnfix,
    GlobalWire,
    InitAtomTable,
    LocalCompact,
    LocalFlags,
    LocalShrink,
    OOBEComplete,
    OpenJobObjectA,
    OpenJobObjectW,
    OpenPrivateNamespaceA,
    OpenPrivateNamespaceW,
    PowerClearRequest,
    PowerCreateRequest,
    PowerSetRequest,
    QueryInformationJobObject,
    QueryIoRateControlInformationJobObject,
    RaiseInvalid16BitExeError,
    RegisterApplicationRecoveryCallback,
    RegisterApplicationRestart,
    RequestWakeupLatency,
    SetComPlusPackageInstallStatus,
    SetInformationJobObject,
    SetIoRateControlInformationJobObject,
    SetLocalPrimaryComputerNameA,
    SetLocalPrimaryComputerNameW,
    UnregisterApplicationRecoveryCallback,
    UnregisterApplicationRestart,
);

unsupported_exports!(
    "kernel32",
    BeepImplementation,
    ByteMatch,
    CreateSocketHandle,
    DeviceNameCompare,
    GetConfigDialogName,
    GetCorruptDetectionState,
    GetCurrentJaEraIndex,
    GetHandleContext,
    GetJapaneseEraItems,
    GetKnownJapanEraCount,
    GetMaxJapanEras,
    GetMaxJapanYears,
    InternalCoStdMarshalObject,
    IsFusionFullySupported,
    IsOOBESupported,
    LoadAppInitDllsImplementation,
    MachinePrefersNumericFirstYear,
    NonInstallerDefault,
    OOBECompleteWnfCallback,
    OOBECompleteWnfQueryCallback,
    ReturnMem16Data,
    SbpIsTraceEnabled,
);
