use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};

use crate::api::abi::windows::{
    ArgumentError,
    CallingConvention::{Stdcall, Thiscall},
    WindowsAbi,
    WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter},
};
use crate::api::windows::common::{nt_api, trace};
use crate::api::windows::signatures;
use crate::arch::Arch;
use crate::config::Config;
use crate::emu::Emu;
use crate::serialization::Serialization;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::{INVALID_SOCKET, SOCKET_ERROR};
use crate::windows::constants::system::{FALSE, TRUE};
use crate::windows::constants::time::TIMERR_NOCANDO;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiMode {
    #[default]
    Strict,
    SimulateFail,
    SimulateSuccess,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiRoute {
    Address,
    Import,
    Gateway,
    Syscall,
    Kernel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiCall {
    pub library: String,
    pub name: String,
    /// Export/stub address, or the raw service number for a syscall.
    pub address: u64,
    pub route: ApiRoute,
    pub pc: u64,
    pub stack_pointer: u64,
    pub thread_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiStopReason {
    StrictPolicy,
    UnknownCleanup,
    InvalidFrame,
    InvalidMemory,
    BanzaiDisabled,
    InvalidArgument(ArgumentError),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiDisposition {
    Stopped(ApiStopReason),
    SimulatedFailure,
    SimulatedSuccess,
    SimulatedGeneric,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiOutcome {
    pub call: ApiCall,
    pub mode: ApiMode,
    pub disposition: ApiDisposition,
    pub result: Option<u64>,
    /// None preserves LastError; Some records the value written by the model.
    pub last_error: Option<u64>,
}

/// Bounded diagnostics and an explicit API policy selection.
/// An unsupported stop remains pending until the caller clears it deliberately.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ApiPolicy {
    pub mode: Option<ApiMode>,
    pub last_outcome: Option<ApiOutcome>,
    pub simulation_count: u64,
    #[serde(skip)]
    pub(crate) entry: Option<ApiCall>,
}

impl ApiPolicy {
    pub fn effective_mode(&self, cfg: &Config) -> ApiMode {
        if cfg.stop_on_unimplemented {
            ApiMode::Strict
        } else {
            self.mode.unwrap_or_default()
        }
    }

    pub fn stopped(&self) -> bool {
        self.last_outcome
            .as_ref()
            .is_some_and(|outcome| matches!(outcome.disposition, ApiDisposition::Stopped(_)))
    }

    /// Clearing a stop does not repair the unsupported call or resume it.
    pub fn clear_outcome(&mut self) {
        self.last_outcome = None;
        self.entry = None;
    }
}

impl Emu {
    /// Select API policy without changing Banzai settings.
    pub fn set_api_mode(&mut self, mode: ApiMode) {
        self.api_policy.mode = Some(mode);
    }

    pub fn api_mode(&self) -> ApiMode {
        self.api_policy.effective_mode(&self.cfg)
    }
}

pub(crate) fn begin_call(emu: &mut Emu, library: &str, name: &str, address: u64, route: ApiRoute) {
    let name = if name.is_empty() {
        emu.export_indexes
            .resolve_address_module(library, address)
            .unwrap_or("")
    } else {
        name
    };
    emu.api_policy.entry = Some(ApiCall {
        library: library.to_string(),
        name: name.to_string(),
        address,
        route,
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    });
}

/// Complete diagnostic bookkeeping; handlers retain ownership of ABI cleanup.
pub(crate) fn finish_call(emu: &mut Emu) {
    if !emu.api_policy.stopped() {
        if emu.cfg.arch.is_x86()
            && !emu
                .api_policy
                .entry
                .as_ref()
                .is_some_and(|entry| matches!(entry.route, ApiRoute::Syscall | ApiRoute::Kernel))
        {
            emu.call_stack_mut().pop();
        }
        emu.api_policy.entry = None;
    }
}

pub(crate) fn enter_gateway(
    emu: &mut Emu,
    library: &str,
    name: &str,
    address: u64,
    route: ApiRoute,
) {
    if emu.api_policy.entry.is_none() {
        begin_call(emu, library, name, address, route);
    }
}

fn call_context(emu: &Emu, library: &str, name: &str, address: u64) -> ApiCall {
    if let Some(mut call) = emu.api_policy.entry.clone() {
        // The interceptor may know only the section until export resolution.
        if call.name.is_empty() {
            call.name = name.to_string();
        }
        return call;
    }
    ApiCall {
        library: library.to_string(),
        name: name.to_string(),
        address,
        route: ApiRoute::Gateway,
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    }
}

fn stop(emu: &mut Emu, call: ApiCall, reason: ApiStopReason) {
    log::error!(
        "stopping at {}!{} (0x{:x}), {:?}: {:?}",
        call.library,
        call.name,
        call.address,
        call.route,
        reason
    );
    // Interception can already have consumed the return address. Preserve the
    // entry frame for inspection, without pretending that the guest returned.
    emu.set_pc(call.pc);
    emu.set_sp(call.stack_pointer);
    emu.api_policy.last_outcome = Some(ApiOutcome {
        call,
        mode: emu.api_mode(),
        disposition: ApiDisposition::Stopped(reason),
        result: None,
        last_error: None,
    });
    emu.force_break = true;
    emu.is_running.store(0, Ordering::Relaxed);
    if emu.cfg.dump_on_exit
        && let Some(filename) = emu.cfg.dump_filename.as_deref()
    {
        Serialization::dump(emu, filename);
    }
}

pub(crate) fn argument_fault(emu: &mut Emu, error: ArgumentError) {
    let context = if !matches!(error.abi, WinApi(_))
        && !emu
            .api_policy
            .entry
            .as_ref()
            .is_some_and(|entry| entry.route == ApiRoute::Syscall)
    {
        ApiCall {
            library: "ntdll".into(),
            name: format!("syscall 0x{:x}", emu.regs().get_eax()),
            address: emu.regs().get_eax(),
            route: ApiRoute::Syscall,
            pc: emu.pc(),
            stack_pointer: emu.sp(),
            thread_id: emu.current_thread_id,
        }
    } else {
        call_context(emu, "unknown", "direct handler", emu.pc())
    };
    stop(emu, context, ApiStopReason::InvalidArgument(error));
}

/// API interception consumes the return address before invoking old handlers.
/// Validate it first and retain the original entry frame for controlled stops.
pub(crate) fn consume_return(emu: &mut Emu) -> bool {
    let return_address = if emu.cfg.arch.is_aarch64() {
        Some((emu.regs_aarch64().x[30], emu.sp()))
    } else if emu.cfg.arch.is_64bits() {
        emu.maps
            .read_qword(emu.sp())
            .zip(emu.sp().checked_add(size_of::<u64>() as u64))
    } else {
        emu.maps.read_dword(emu.sp()).map(u64::from).zip(
            emu.sp()
                .checked_add(size_of::<u32>() as u64)
                .filter(|sp| *sp <= u32::MAX as u64),
        )
    };
    if let Some((address, sp)) = return_address {
        emu.gateway_return = address;
        emu.set_sp(sp);
        emu.set_pc(address);
        true
    } else {
        let call = call_context(emu, "unknown", "unknown", 0);
        stop(emu, call, ApiStopReason::InvalidFrame);
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Simulation {
    MessageUnavailable,
    TimerFailure,
    StatusFailure,
    StatusSuccess,
    NtFailure,
    NtSuccess,
    KernelBoolean(bool),
    Win32Failure,
    Win32Success,
    HresultFailure,
    HresultSuccess,
    GenericZero,
    Void,
    WinsockFailure,
    SocketFailure,
    WinsockSuccess,
}

pub(super) enum ReturnType {
    GetMessage,
    Generic,
    Void,
    TimerStatus,
    Win32Status,
    Handle,
    WinsockStatus,
    Hresult,
    Win32Bool,
    NtStatus,
}

struct Recipe {
    x86_words: u32,
    arguments: u32,
    simulation: Simulation,
}

fn recipe(library: &str, name: &str, mode: ApiMode, route: ApiRoute) -> Option<Recipe> {
    if let Some(recipe) = builtin_recipe(library, name, mode, route) {
        return Some(recipe);
    }
    if matches!(route, ApiRoute::Kernel | ApiRoute::Syscall) {
        return None;
    }
    let signature = signatures::lookup(library, name)?;
    simulated_recipe(
        mode,
        signature.arguments,
        signature.x86_words,
        signature.result,
    )
}

fn builtin_recipe(library: &str, name: &str, mode: ApiMode, route: ApiRoute) -> Option<Recipe> {
    let library = library.trim_end_matches(".dll");
    if (matches!(route, ApiRoute::Syscall | ApiRoute::Kernel)
        || ["ntdll", "kernel32", "kernelbase"].contains(&library))
        && let Some(signature) = nt_api::signature(name)
    {
        let simulation = match mode {
            ApiMode::SimulateFail => Simulation::NtFailure,
            ApiMode::SimulateSuccess => Simulation::NtSuccess,
            ApiMode::Strict => return None,
        };
        return Some(Recipe {
            x86_words: signature.x86_words,
            arguments: signature.arguments,
            simulation,
        });
    }
    if route == ApiRoute::Kernel {
        return wdm_recipe(name, mode);
    }
    if library == "ws2_32" {
        let (arguments, socket_result) = match name {
            "accept" => (3, true),
            "WSAAccept" => (5, true),
            "WSASocketA" | "WSASocketW" => (6, true),
            "recv" => (4, false),
            "WSAEventSelect" => (3, false),
            "WSAConnect" | "WSASend" | "WSARecv" => (7, false),
            "WSASendTo" | "WSARecvFrom" | "WSAIoctl" => (9, false),
            "WSASendMsg" => (6, false),
            _ => return None,
        };
        let simulation = match mode {
            ApiMode::Strict => return None,
            ApiMode::SimulateFail if socket_result => Simulation::SocketFailure,
            ApiMode::SimulateFail => Simulation::WinsockFailure,
            ApiMode::SimulateSuccess => Simulation::WinsockSuccess,
        };
        return Some(Recipe {
            x86_words: arguments,
            arguments,
            simulation,
        });
    }
    // These operations have no object model yet. Keep their known return/ABI
    // contracts here so a constant success is never mistaken for a handler.
    let (arguments, x86_words, contract) = match (library, name) {
        ("advapi32" | "ntdll", "MD4Init" | "MD4Final" | "MD5Init" | "MD5Final") => {
            (1, 1, ReturnType::Void)
        }
        ("advapi32" | "ntdll", "MD4Update" | "MD5Update") => (3, 3, ReturnType::Void),
        ("advapi32", "BaseRegGetVersion" | "BaseRegUnLoadKey") => (2, 2, ReturnType::Win32Status),
        ("advapi32", "BaseRegLoadKey" | "BaseRegRestoreKey" | "BaseRegSetKeySecurity") => {
            (3, 3, ReturnType::Win32Status)
        }
        ("advapi32", "BaseRegSaveKeyEx") => (4, 4, ReturnType::Win32Status),
        ("advapi32", "BaseRegSetValue") => (5, 5, ReturnType::Win32Status),
        ("advapi32", "BaseRegCreateKey") => (8, 8, ReturnType::Win32Status),
        ("advapi32", "EncryptionDisable") => (2, 2, ReturnType::Win32Bool),
        ("advapi32", "LsaDelete") => (1, 1, ReturnType::NtStatus),
        ("advapi32", "LsaDeleteTrustedDomain") => (2, 2, ReturnType::NtStatus),
        ("advapi32", "LsaRemoveAccountRights") => (5, 5, ReturnType::NtStatus),
        ("advapi32", "LsaRemovePrivilegesFromAccount") => (3, 3, ReturnType::NtStatus),
        ("advapi32", "PerfDeleteCounters") => (3, 3, ReturnType::Win32Status),
        ("advapi32", "PerfDeleteInstance") => (2, 2, ReturnType::Win32Status),
        ("advapi32", "RemoveTraceCallback") => (1, 1, ReturnType::Win32Status),
        ("advapi32", "RemoveUsersFromEncryptedFile") => (2, 2, ReturnType::Win32Status),
        ("advapi32", "UninstallApplication") => (2, 2, ReturnType::Win32Status),
        ("advapi32", "UnregisterTraceGuids") => (1, 2, ReturnType::Win32Status),
        ("crypt32", "CryptSIPAddProvider") => (1, 1, ReturnType::Win32Bool),
        ("crypt32", "CryptSIPRemoveProvider") => (1, 1, ReturnType::Win32Bool),
        ("advapi32" | "cryptbase", "SystemFunction040" | "SystemFunction041") => {
            (3, 3, ReturnType::NtStatus)
        }
        ("advapi32", "CreateServiceA" | "CreateServiceW") => (13, 13, ReturnType::Handle),
        ("libgcc_s_dw2-1", "__register_frame_info") => (2, 2, ReturnType::Void),
        ("libgcc_s_dw2-1", "__deregister_frame_info") => (1, 1, ReturnType::Generic),
        ("shell32", "DllGetVersion") => (1, 1, ReturnType::Hresult),
        (
            "ntdll" | "kernel32" | "kernelbase" | "advapi32",
            "EtwEventRegister" | "EventRegister",
        ) => (4, 4, ReturnType::Win32Status),
        (
            "ntdll" | "kernel32" | "kernelbase" | "advapi32",
            "EtwEventSetInformation" | "EventSetInformation",
        ) => (4, 5, ReturnType::Win32Status),
        (
            "ntdll" | "kernel32" | "kernelbase" | "advapi32",
            "EtwEventWriteTransfer" | "EventWriteTransfer",
        ) => (6, 7, ReturnType::Win32Status),
        (
            "ntdll" | "kernel32" | "kernelbase" | "advapi32",
            "EtwEventUnregister" | "EventUnregister",
        ) => (1, 2, ReturnType::Win32Status),
        ("kernel32" | "kernelbase" | "winmm", "timeBeginPeriod" | "timeEndPeriod") => {
            (1, 1, ReturnType::TimerStatus)
        }
        ("ole32", "CoMarshalInterface" | "CoGetMarshalSizeMax") => (6, 6, ReturnType::Hresult),
        ("ole32", "OleConvertOLESTREAMToIStorageEx") => (7, 7, ReturnType::Hresult),
        ("ole32", "OleConvertOLESTREAMToIStorageEx2") => (10, 10, ReturnType::Hresult),
        ("crypt32", "CertVerifyRevocation" | "CryptDecodeObject") => (7, 7, ReturnType::Win32Bool),
        ("crypt32", "CryptDecodeObjectEx") => (8, 8, ReturnType::Win32Bool),
        ("crypt32" | "wintrust", "CryptSIPCreateIndirectData" | "CryptSIPLoad") => {
            (3, 3, ReturnType::Win32Bool)
        }
        ("user32", "GetScrollInfo") => (3, 3, ReturnType::Win32Bool),
        (
            "kernel32" | "kernelbase",
            "UnregisterApplicationRecoveryCallback" | "UnregisterApplicationRestart",
        ) => (0, 0, ReturnType::Hresult),
        ("kernel32" | "kernelbase", "PowerCreateRequest") => (1, 1, ReturnType::Handle),
        ("kernel32" | "kernelbase", "ApplicationRecoveryInProgress" | "WerSetFlags") => {
            (1, 1, ReturnType::Hresult)
        }
        (
            "kernel32" | "kernelbase",
            "DebugSetProcessKillOnExit"
            | "DeleteTimerQueue"
            | "GetFirmwareType"
            | "GetSystemPowerStatus"
            | "GlobalUnWire"
            | "Heap32Next"
            | "InitAtomTable"
            | "OOBEComplete"
            | "RemoveSecureMemoryCacheCallback"
            | "RequestWakeupLatency"
            | "SetProcessDEPPolicy"
            | "SetUserGeoID"
            | "SetUserGeoName"
            | "UnregisterWait"
            | "UnregisterWaitUntilOOBECompleted",
        ) => (1, 1, ReturnType::Win32Bool),
        ("kernel32" | "kernelbase", "EnableThreadProfiling") => (4, 5, ReturnType::Win32Status),
        ("kernel32" | "kernelbase", "DisableThreadProfiling" | "GetTapeStatus") => {
            (1, 1, ReturnType::Win32Status)
        }
        ("kernel32" | "kernelbase", "OpenPrivateNamespaceA") => (2, 2, ReturnType::Handle),
        ("kernel32" | "kernelbase", "RegisterApplicationRestart") => (2, 2, ReturnType::Hresult),
        (
            "kernel32" | "kernelbase",
            "AddIntegrityLabelToBoundaryDescriptor"
            | "CancelTimerQueueTimer"
            | "EnumSystemCodePagesA"
            | "GetDevicePowerState"
            | "GetNumaAvailableMemoryNode"
            | "GetNumaAvailableMemoryNodeEx"
            | "GetNumaNodeNumberFromHandle"
            | "GetNumaNodeProcessorMask"
            | "GetNumaProcessorNode"
            | "GetNumaProcessorNodeEx"
            | "GetNumaProximityNode"
            | "GetProcessIoCounters"
            | "GetSystemRegistryQuota"
            | "Heap32ListFirst"
            | "Heap32ListNext"
            | "PowerClearRequest"
            | "PowerSetRequest"
            | "SetMessageWaitingIndicator"
            | "SetSystemPowerState"
            | "SetSystemTimeAdjustment",
        ) => (2, 2, ReturnType::Win32Bool),
        (
            "kernel32" | "kernelbase",
            "QueryThreadProfiling" | "SetIoRateControlInformationJobObject",
        ) => (2, 2, ReturnType::Win32Status),
        (
            "kernel32" | "kernelbase",
            "BindIoCompletionCallback"
            | "EnumDateFormatsA"
            | "EnumDateFormatsExA"
            | "EnumSystemGeoID"
            | "EnumSystemGeoNames"
            | "EnumSystemLanguageGroupsA"
            | "EnumTimeFormatsA"
            | "EnumUILanguagesA"
            | "GetProcessDEPPolicy"
            | "GetThreadSelectorEntry"
            | "Heap32First"
            | "RegisterWaitUntilOOBECompleted"
            | "ReplacePartitionUnit"
            | "Wow64GetThreadSelectorEntry",
        ) => (3, 3, ReturnType::Win32Bool),
        (
            "kernel32" | "kernelbase",
            "EraseTape" | "PrepareTape" | "ReadThreadProfilingData" | "SetTapeParameters",
        ) => (3, 3, ReturnType::Win32Status),
        ("kernel32" | "kernelbase", "RegisterApplicationRecoveryCallback") => {
            (4, 4, ReturnType::Hresult)
        }
        (
            "kernel32" | "kernelbase",
            "EnumCalendarInfoA"
            | "EnumCalendarInfoExA"
            | "EnumLanguageGroupLocalesA"
            | "SetCalendarInfoA"
            | "SetInformationJobObject",
        ) => (4, 4, ReturnType::Win32Bool),
        (
            "kernel32" | "kernelbase",
            "CreateTapePartition"
            | "GetTapeParameters"
            | "QueryIoRateControlInformationJobObject"
            | "WriteTapemark",
        ) => (4, 4, ReturnType::Win32Status),
        ("kernel32" | "kernelbase", "NotifyUILanguageChange" | "QueryInformationJobObject") => {
            (5, 5, ReturnType::Win32Bool)
        }
        ("kernel32" | "kernelbase", "GetTapePosition") => (5, 5, ReturnType::Win32Status),
        ("kernel32" | "kernelbase", "SetTimerQueueTimer") => (6, 6, ReturnType::Handle),
        ("kernel32" | "kernelbase", "SetTapePosition") => (6, 6, ReturnType::Win32Status),
        ("kernel32" | "kernelbase", "LockFileEx") => (6, 6, ReturnType::Win32Bool),
        ("ole32", "OleConvertIStorageToOLESTREAMEx") => (7, 7, ReturnType::Hresult),
        (
            "oleaut32",
            "BstrFromVector"
            | "GetAltMonthNames"
            | "GetRecordInfoFromTypeInfo"
            | "RevokeActiveObject"
            | "SafeArrayAddRef"
            | "SafeArrayRedim"
            | "VarAbs"
            | "VarCyFromDec"
            | "VarDateFromDec"
            | "VarFix"
            | "VarInt"
            | "VarNeg"
            | "VarNot"
            | "VectorFromBstr",
        ) => (2, 2, ReturnType::Hresult),
        ("oleaut32", "OleLoadPictureFile") => (2, 5, ReturnType::Hresult),
        (
            "oleaut32",
            "OleCreateFontIndirect"
            | "OleTranslateColor"
            | "VarAdd"
            | "VarAnd"
            | "VarBstrCat"
            | "VarCat"
            | "VarCyFromDisp"
            | "VarDateFromDisp"
            | "VarDateFromUdate"
            | "VarDecAdd"
            | "VarDecDiv"
            | "VarDecMul"
            | "VarDecSub"
            | "VarDiv"
            | "VarEqv"
            | "VarIdiv"
            | "VarImp"
            | "VarMod"
            | "VarMul"
            | "VarOr"
            | "VarPow"
            | "VarRound"
            | "VarSub"
            | "VarXor",
        ) => (3, 3, ReturnType::Hresult),
        (
            "oleaut32",
            "DispGetIDsOfNames"
            | "OleCreatePictureIndirect"
            | "RegisterActiveObject"
            | "VarCyFromStr"
            | "VarDateFromStr"
            | "VarDateFromUdateEx"
            | "VarFormatDateTime"
            | "VarMonthName"
            | "VarNumFromParseNum",
        ) => (4, 4, ReturnType::Hresult),
        (
            "oleaut32",
            "DispGetParam"
            | "OleLoadPicture"
            | "QueryPathOfRegTypeLib"
            | "VarParseNumFromStr"
            | "VarWeekdayName",
        ) => (5, 5, ReturnType::Hresult),
        ("oleaut32", "OleLoadPictureFileEx") => (5, 8, ReturnType::Hresult),
        (
            "oleaut32",
            "GetRecordInfoFromGuids" | "OleLoadPicturePath" | "VarFormat" | "VarFormatFromTokens",
        ) => (6, 6, ReturnType::Hresult),
        (
            "oleaut32",
            "VarFormatCurrency"
            | "VarFormatNumber"
            | "VarFormatPercent"
            | "VarTokenizeFormatString",
        ) => (7, 7, ReturnType::Hresult),
        ("oleaut32", "DispCallFunc" | "OleLoadPictureEx") => (8, 8, ReturnType::Hresult),
        ("shell32", "SHGetInstanceExplorer" | "SHGetMalloc" | "SHQueryUserNotificationState") => {
            (1, 1, ReturnType::Hresult)
        }
        (
            "shell32",
            "ILLoadFromStreamEx"
            | "SHGetDriveMedia"
            | "SHIsFileAvailableOffline"
            | "Shell_NotifyIconGetRect",
        ) => (2, 2, ReturnType::Hresult),
        ("shell32", "SHGetSetFolderCustomSettings" | "SHGetTemporaryPropertyForItem") => {
            (3, 3, ReturnType::Hresult)
        }
        ("shell32", "SHEvaluateSystemCommandTemplate" | "SHGetLocalizedName") => {
            (4, 4, ReturnType::Hresult)
        }
        (
            "shell32",
            "AssocGetDetailsOfPropKey"
            | "SHDoDragDrop"
            | "SHPropStgReadMultiple"
            | "StgMakeUniqueName",
        ) => (5, 5, ReturnType::Hresult),
        ("shell32", "SHGetUnreadMailCountW") => (6, 6, ReturnType::Hresult),
        ("shell32", "SHPropStgCreate") => (8, 8, ReturnType::Hresult),
        ("advapi32", "AuditEnumeratePerUserPolicy") => (1, 1, ReturnType::Win32Bool),
        (
            "advapi32",
            "AuditEnumerateCategories"
            | "AuditLookupCategoryGuidFromCategoryId"
            | "AuditLookupCategoryIdFromCategoryGuid"
            | "AuditQuerySecurity"
            | "CredGetSessionTypes",
        ) => (2, 2, ReturnType::Win32Bool),
        (
            "advapi32",
            "LsaGetAppliedCAPIDs"
            | "LsaQueryDomainInformationPolicy"
            | "LsaQueryForestTrustInformation"
            | "LsaQueryInformationPolicy"
            | "LsaRetrievePrivateData",
        ) => (3, 3, ReturnType::NtStatus),
        (
            "advapi32",
            "AuditQuerySystemPolicy" | "PrivilegeCheck" | "QueryServiceDynamicInformation",
        ) => (3, 3, ReturnType::Win32Bool),
        (
            "advapi32",
            "LsaEnumerateAccountRights"
            | "LsaEnumerateAccountsWithUserRight"
            | "LsaOpenPolicy"
            | "LsaOpenTrustedDomainByName"
            | "LsaQueryCAPs"
            | "LsaQueryForestTrustInformation2"
            | "LsaQueryTrustedDomainInfo"
            | "LsaQueryTrustedDomainInfoByName",
        ) => (4, 4, ReturnType::NtStatus),
        (
            "advapi32",
            "AuditComputeEffectivePolicyBySid"
            | "AuditComputeEffectivePolicyByToken"
            | "AuditEnumerateSubCategories"
            | "AuditQueryPerUserPolicy"
            | "SaferIdentifyLevel",
        ) => (4, 4, ReturnType::Win32Bool),
        (
            "advapi32",
            "LsaCreateTrustedDomainEx"
            | "LsaEnumerateTrustedDomains"
            | "LsaEnumerateTrustedDomainsEx"
            | "LsaLookupNames"
            | "LsaLookupSids",
        ) => (5, 5, ReturnType::NtStatus),
        (
            "advapi32",
            "QueryServiceObjectSecurity"
            | "SaferComputeTokenFromLevel"
            | "SaferCreateLevel"
            | "SaferGetLevelInformation",
        ) => (5, 5, ReturnType::Win32Bool),
        ("advapi32", "LsaLookupNames2" | "LsaLookupSids2") => (6, 6, ReturnType::NtStatus),
        ("advapi32", "SaferGetPolicyInformation") => (6, 6, ReturnType::Win32Bool),
        ("advapi32", "AccessCheck") => (8, 8, ReturnType::Win32Bool),
        ("advapi32", "AccessCheckByType" | "AccessCheckByTypeResultList") => {
            (11, 11, ReturnType::Win32Bool)
        }
        ("crypt32", "CryptUnprotectMemory") => (3, 3, ReturnType::Win32Bool),
        ("crypt32", "CryptEnumOIDInfo") => (4, 4, ReturnType::Win32Bool),
        ("crypt32", "CryptImportPublicKeyInfoEx2") => (5, 5, ReturnType::Win32Bool),
        ("crypt32", "CryptEnumOIDFunction") => (6, 6, ReturnType::Win32Bool),
        (
            "crypt32",
            "CertVerifyCTLUsage"
            | "CryptEncryptMessage"
            | "CryptEnumKeyIdentifierProperties"
            | "CryptHashPublicKeyInfo"
            | "CryptImportPublicKeyInfoEx",
        ) => (7, 7, ReturnType::Win32Bool),
        (
            "crypt32",
            "CertSelectCertificateChains"
            | "CryptVerifyCertificateSignatureEx"
            | "CryptVerifyDetachedMessageSignature",
        ) => (8, 8, ReturnType::Win32Bool),
        ("crypt32", "CryptMsgVerifyCountersignatureEncodedEx") => (10, 10, ReturnType::Win32Bool),
        ("kernelbase", "PrivilegeCheck") => (3, 3, ReturnType::Win32Bool),
        ("kernelbase", "AccessCheck") => (8, 8, ReturnType::Win32Bool),
        ("kernelbase", "AccessCheckByType" | "AccessCheckByTypeResultList") => {
            (11, 11, ReturnType::Win32Bool)
        }
        (
            "user32",
            "GetGUIThreadInfo"
            | "GetGestureInfo"
            | "GetIconInfo"
            | "GetMenuInfo"
            | "GetPointerCursorId"
            | "GetPointerDevice"
            | "GetPointerDevices"
            | "GetPointerInfo"
            | "GetPointerPenInfo"
            | "GetPointerTouchInfo"
            | "GetPointerType"
            | "GetTitleBarInfo"
            | "GetWindowInfo",
        ) => (2, 2, ReturnType::Win32Bool),
        (
            "user32",
            "GetGestureExtraArgs"
            | "GetPointerDeviceCursors"
            | "GetPointerDeviceProperties"
            | "GetPointerDeviceRects"
            | "GetPointerFrameInfo"
            | "GetPointerFramePenInfo"
            | "GetPointerFrameTouchInfo"
            | "GetPointerInfoHistory"
            | "GetPointerInputTransform"
            | "GetPointerPenInfoHistory"
            | "GetPointerTouchInfoHistory"
            | "GetScrollBarInfo"
            | "GetUpdatedClipboardFormats",
        ) => (3, 3, ReturnType::Win32Bool),
        (
            "user32",
            "GetLayeredWindowAttributes"
            | "GetMenuBarInfo"
            | "GetMenuItemRect"
            | "GetPointerFrameInfoHistory"
            | "GetPointerFramePenInfoHistory"
            | "GetPointerFrameTouchInfoHistory"
            | "GetTouchInputInfo",
        ) => (4, 4, ReturnType::Win32Bool),
        (
            "user32",
            "GetRawPointerDeviceData" | "GetUserObjectSecurity" | "GetWindowFeedbackSetting",
        ) => (5, 5, ReturnType::Win32Bool),
        ("user32", "GetGestureConfig") => (6, 6, ReturnType::Win32Bool),
        (
            "wininet",
            "GetUrlCacheHeaderData" | "IncrementUrlCacheHeaderData" | "InternetQueryFortezzaStatus",
        ) => (2, 2, ReturnType::Win32Bool),
        ("user32", "GetDCEx") => (3, 3, ReturnType::Handle),
        ("gdi32", "CreateDCA" | "CreateDCW") => (4, 4, ReturnType::Handle),
        ("winhttp", "WinHttpSendRequest") => (7, 7, ReturnType::Win32Bool),
        ("winhttp", "WinHttpReceiveResponse") => (2, 2, ReturnType::Win32Bool),
        ("winhttp", "WinHttpReadData")
        | ("wininet", "InternetReadFile" | "InternetReadFileExA" | "InternetReadFileExW") => {
            (4, 4, ReturnType::Win32Bool)
        }
        ("wininet", "ReadUrlCacheEntryStreamEx") => (4, 5, ReturnType::Win32Bool),
        ("wininet", "HttpWebSocketReceive" | "ReadUrlCacheEntryStream") => {
            (5, 5, ReturnType::Win32Bool)
        }
        ("kernel32" | "kernelbase", name) => kernel_contract(name)?,
        _ => return None,
    };
    simulated_recipe(mode, arguments, x86_words, contract)
}

fn wdm_recipe(name: &str, mode: ApiMode) -> Option<Recipe> {
    if matches!(name, "IoForwardIrpSynchronously" | "IoForwardAndCatchIrp") {
        let value = match mode {
            ApiMode::Strict => return None,
            ApiMode::SimulateFail => false,
            ApiMode::SimulateSuccess => true,
        };
        return Some(Recipe {
            arguments: 2,
            x86_words: 2,
            simulation: Simulation::KernelBoolean(value),
        });
    }
    let arguments = match name {
        "IoRegisterLastChanceShutdownNotification"
        | "IoRegisterShutdownNotification"
        | "IoWMIWriteEvent" => 1,
        "IoGetDeviceNumaNode"
        | "IoReadPartitionTableEx"
        | "IoReportTargetDeviceChange"
        | "IoSetDeviceInterfaceState"
        | "IoSetFileOrigin"
        | "IoSetIoPriorityHint"
        | "IoValidateDeviceIoControlAccess"
        | "IoVerifyPartitionTable"
        | "IoVerifyVolume"
        | "IoWMIRegistrationControl" => 2,
        "IoCheckEaBufferValidity"
        | "IoCheckQuerySetFileInformation"
        | "IoCheckQuerySetVolumeInformation"
        | "IoCheckQuotaBufferValidity"
        | "IoGetDeviceInterfaceAlias"
        | "IoReplacePartitionUnit"
        | "IoWMIAllocateInstanceIds"
        | "IoWMIDeviceObjectToInstanceName"
        | "IoWMIHandleToInstanceName"
        | "IoWMIOpenBlock"
        | "IoWMIQueryAllData"
        | "IoWMISetNotificationCallback" => 3,
        "IoEnumerateDeviceObjectList"
        | "IoGetDeviceInterfaces"
        | "IoReadPartitionTable"
        | "IoRegisterDeviceInterface"
        | "IoReportTargetDeviceChangeAsynchronous"
        | "IoSetInformation"
        | "IoSetPartitionInformation"
        | "IoWMIQuerySingleInstance"
        | "IoWMISuggestInstanceName" => 4,
        "IoAcquireRemoveLockEx"
        | "IoCheckShareAccess"
        | "IoGetDeviceProperty"
        | "IoRegisterContainerNotification"
        | "IoWritePartitionTable" => 5,
        "IoAssignResources" | "IoCheckShareAccessEx" => 6,
        "IoCsqInitialize"
        | "IoCsqInitializeEx"
        | "IoRegisterPlugPlayNotification"
        | "IoReportResourceForDetection"
        | "IoSetCompletionRoutineEx"
        | "IoSetDevicePropertyData" => 7,
        "IoGetDevicePropertyData" | "IoReportDetectedDevice" => 8,
        "IoReportResourceUsage" => 9,
        _ => return None,
    };
    simulated_recipe(mode, arguments, arguments, ReturnType::NtStatus)
}

fn simulated_recipe(
    mode: ApiMode,
    arguments: u32,
    x86_words: u32,
    contract: ReturnType,
) -> Option<Recipe> {
    let simulation = match (mode, contract) {
        (ApiMode::SimulateFail | ApiMode::SimulateSuccess, ReturnType::GetMessage) => {
            Simulation::MessageUnavailable
        }
        (ApiMode::SimulateFail | ApiMode::SimulateSuccess, ReturnType::Generic) => {
            Simulation::GenericZero
        }
        (ApiMode::SimulateFail, ReturnType::WinsockStatus) => Simulation::WinsockFailure,
        (ApiMode::SimulateSuccess, ReturnType::WinsockStatus) => Simulation::WinsockSuccess,
        (ApiMode::SimulateFail, ReturnType::Handle) => Simulation::Win32Failure,
        (ApiMode::SimulateSuccess, ReturnType::Handle) => Simulation::GenericZero,
        (ApiMode::SimulateFail, ReturnType::Hresult) => Simulation::HresultFailure,
        (ApiMode::SimulateSuccess, ReturnType::Hresult) => Simulation::HresultSuccess,
        (ApiMode::SimulateFail, ReturnType::Win32Bool) => Simulation::Win32Failure,
        (ApiMode::SimulateSuccess, ReturnType::Win32Bool) => Simulation::Win32Success,
        (ApiMode::SimulateFail, ReturnType::NtStatus) => Simulation::NtFailure,
        (ApiMode::SimulateSuccess, ReturnType::NtStatus) => Simulation::NtSuccess,
        (ApiMode::SimulateFail, ReturnType::TimerStatus) => Simulation::TimerFailure,
        (ApiMode::SimulateSuccess, ReturnType::TimerStatus) => Simulation::StatusSuccess,
        (ApiMode::SimulateFail, ReturnType::Win32Status) => Simulation::StatusFailure,
        (ApiMode::SimulateSuccess, ReturnType::Win32Status) => Simulation::StatusSuccess,
        (ApiMode::SimulateFail | ApiMode::SimulateSuccess, ReturnType::Void) => Simulation::Void,
        (ApiMode::Strict, _) => return None,
    };
    Some(Recipe {
        x86_words,
        arguments,
        simulation,
    })
}

fn kernel_contract(name: &str) -> Option<(u32, u32, ReturnType)> {
    Some(match name {
        "WerUnregisterAppLocalDump" => (0, 0, ReturnType::Hresult),
        "DeletePackageDependency"
        | "GetGamingDeviceModelInformation"
        | "GetIntegratedDisplaySize"
        | "QueryAuxiliaryCounterFrequency"
        | "RemovePackageDependency"
        | "WerRegisterAppLocalDump"
        | "WerUnregisterAdditionalProcess"
        | "WerUnregisterCustomMetadata"
        | "WerUnregisterExcludedMemoryBlock"
        | "WerUnregisterFile"
        | "WerUnregisterMemoryBlock" => (1, 1, ReturnType::Hresult),
        "ActivatePackageVirtualizationContext"
        | "CheckIsMSIXPackage"
        | "CreatePackageVirtualizationContext"
        | "DuplicatePackageVirtualizationContext"
        | "GetAcceptLanguagesA"
        | "GetAcceptLanguagesW"
        | "GetIdForPackageDependencyContext"
        | "GetResolvedPackageFullNameForPackageDependency"
        | "ParseURLA"
        | "ParseURLW"
        | "ResizePseudoConsole"
        | "WerGetFlags"
        | "WerRegisterAdditionalProcess"
        | "WerRegisterCustomMetadata"
        | "WerRegisterExcludedMemoryBlock"
        | "WerRegisterMemoryBlock"
        | "WerRegisterRuntimeExceptionModule"
        | "WerUnregisterRuntimeExceptionModule" => (2, 2, ReturnType::Hresult),
        "GetProcessesInVirtualizationContext"
        | "UrlFixupW"
        | "UrlHashA"
        | "UrlHashW"
        | "WerRegisterFile" => (3, 3, ReturnType::Hresult),
        "ConvertAuxiliaryCounterToPerformanceCounter"
        | "ConvertPerformanceCounterToAuxiliaryCounter" => (3, 4, ReturnType::Hresult),
        "GetApplicationRestartSettings"
        | "HashData"
        | "QISearch"
        | "SHLoadIndirectString"
        | "UrlApplySchemeA"
        | "UrlApplySchemeW"
        | "UrlCanonicalizeA"
        | "UrlCanonicalizeW"
        | "UrlCreateFromPathA"
        | "UrlCreateFromPathW"
        | "UrlEscapeA"
        | "UrlEscapeW"
        | "UrlUnescapeA"
        | "UrlUnescapeW" => (4, 4, ReturnType::Hresult),
        "AddPackageDependency"
        | "CreatePseudoConsole"
        | "GetApplicationRecoveryCallback"
        | "SHCoCreateInstance"
        | "UrlCombineA"
        | "UrlCombineW"
        | "UrlGetPartA"
        | "UrlGetPartW" => (5, 5, ReturnType::Hresult),
        "TryCreatePackageDependency" => (8, 9, ReturnType::Hresult),
        "FlushConsoleInputBuffer"
        | "GetConsoleDisplayMode"
        | "GetConsoleHistoryInfo"
        | "GetConsoleSelectionInfo"
        | "SetConsoleHistoryInfo"
        | "SetConsoleTitleA"
        | "SetConsoleTitleW" => (1, 1, ReturnType::Win32Bool),
        "GetCommMask"
        | "GetCommState"
        | "GetCommTimeouts"
        | "GetConsoleScreenBufferInfo"
        | "GetConsoleScreenBufferInfoEx"
        | "GetProcessHandleCount"
        | "GetProcessPriorityBoost"
        | "GetThreadPriorityBoost"
        | "PurgeComm"
        | "SetCommMask"
        | "SetCommState"
        | "SetCommTimeouts"
        | "SetConsoleCursorPosition"
        | "SetConsoleScreenBufferInfoEx"
        | "SetProcessPriorityBoost"
        | "SetThreadPriorityBoost" => (2, 2, ReturnType::Win32Bool),
        "ClearCommError"
        | "GetCurrentConsoleFont"
        | "GetCurrentConsoleFontEx"
        | "SetConsoleDisplayMode"
        | "SetCurrentConsoleFontEx"
        | "SetupComm" => (3, 3, ReturnType::Win32Bool),
        "ReadConsoleOutputA"
        | "ReadConsoleOutputW"
        | "WriteConsoleOutputA"
        | "WriteConsoleOutputW" => (5, 5, ReturnType::Win32Bool),
        "RegDisablePredefinedCacheEx" => (0, 0, ReturnType::Win32Status),
        "SHRegCloseUSKey"
        | "VerifyApplicationUserModelId"
        | "VerifyPackageFamilyName"
        | "VerifyPackageFullName"
        | "VerifyPackageId"
        | "VerifyPackageRelativeApplicationId" => (1, 1, ReturnType::Win32Status),
        "GetStagedPackageOrigin" | "RegUnLoadKeyA" | "RegUnLoadKeyW" => {
            (2, 2, ReturnType::Win32Status)
        }
        "GetApplicationUserModelIdFromToken"
        | "GetCurrentPackagePath2"
        | "GetPackageFamilyNameFromToken"
        | "GetPackageFullNameFromToken"
        | "RegCopyTreeW"
        | "RegDeleteKeyValueA"
        | "RegDeleteKeyValueW"
        | "RegLoadKeyA"
        | "RegLoadKeyW"
        | "RegRestoreKeyA"
        | "RegRestoreKeyW"
        | "RegSetKeySecurity"
        | "SHRegDeleteEmptyUSKeyA"
        | "SHRegDeleteEmptyUSKeyW"
        | "SHRegDeleteUSValueA"
        | "SHRegDeleteUSValueW" => (3, 3, ReturnType::Win32Status),
        "GetPackagePathByFullName2"
        | "GetStagedPackagePathByFullName2"
        | "OpenPackageInfoByFullNameForUser"
        | "RegGetKeySecurity"
        | "RegOpenUserClassesRoot"
        | "RegSaveKeyExA"
        | "RegSaveKeyExW" => (4, 4, ReturnType::Win32Status),
        "GetCurrentPackageInfo2"
        | "RegLoadAppKeyA"
        | "RegLoadAppKeyW"
        | "RegNotifyChangeKeyValue"
        | "RegQueryMultipleValuesA"
        | "RegQueryMultipleValuesW"
        | "SHRegCreateUSKeyA"
        | "SHRegCreateUSKeyW"
        | "SHRegEnumUSKeyA"
        | "SHRegEnumUSKeyW"
        | "SHRegOpenUSKeyA"
        | "SHRegOpenUSKeyW" => (5, 5, ReturnType::Win32Status),
        "GetPackageInfo2"
        | "RegSetKeyValueA"
        | "RegSetKeyValueW"
        | "SHRegQueryInfoUSKeyA"
        | "SHRegQueryInfoUSKeyW"
        | "SHRegSetUSValueA"
        | "SHRegSetUSValueW"
        | "SHRegWriteUSValueA"
        | "SHRegWriteUSValueW" => (6, 6, ReturnType::Win32Status),
        "RegLoadMUIStringA" | "RegLoadMUIStringW" => (7, 7, ReturnType::Win32Status),
        "SHRegEnumUSValueA" | "SHRegEnumUSValueW" | "SHRegGetUSValueA" | "SHRegGetUSValueW"
        | "SHRegQueryUSValueA" | "SHRegQueryUSValueW" => (8, 8, ReturnType::Win32Status),
        "GetRegistryValueWithFallbackW" => (10, 10, ReturnType::Win32Status),
        _ => return None,
    })
}

fn generic_recipe(emu: &mut Emu, name: &str, call: WindowsAbi) -> Result<Recipe, ApiStopReason> {
    let x86_words = if emu.cfg.arch.is_64bits() || !matches!(call, WinApi(Stdcall | Thiscall)) {
        0
    } else if name.starts_with("Nt") || name.starts_with("Zw") {
        // Fallback metadata cannot override the subsystem's signature checks.
        nt_api::signature(name)
            .ok_or(ApiStopReason::UnknownCleanup)?
            .x86_words
    } else {
        if !emu.cfg.skip_unimplemented {
            return Err(ApiStopReason::BanzaiDisabled);
        }
        if emu.banzai.stack_words(name).is_none() {
            // Also support enabling Banzai after initialization.
            emu.banzai
                .load_defaults(&emu.cfg.get_maps_folder("banzai.csv"));
        }
        emu.banzai
            .stack_words(name)
            .ok_or(ApiStopReason::UnknownCleanup)?
    };
    Ok(Recipe {
        x86_words,
        // Generic native calls do not inspect arguments or output pointers.
        arguments: 0,
        simulation: Simulation::GenericZero,
    })
}

fn trace_signature(
    emu: &Emu,
    call: WindowsAbi,
    context: &ApiCall,
    signature: &Recipe,
    declared: bool,
) {
    let parameters = declared.then_some(signature.arguments);
    trace::unsupported(
        emu,
        call,
        &context.library,
        &context.name,
        parameters,
        signature.x86_words,
    );
}

fn checked_frame(emu: &Emu, recipe: &Recipe, call: WindowsAbi) -> Option<u64> {
    if (call == WinSyscall && !emu.cfg.arch.is_x64())
        || (matches!(call, WinSysenter | WinInterrupt) && emu.cfg.arch != Arch::X86)
    {
        return None;
    }
    let (next_sp, slots) = if emu.cfg.arch.is_64bits() {
        (emu.sp(), recipe.arguments)
    } else if !matches!(call, WinApi(Stdcall | Thiscall)) {
        (emu.sp(), recipe.x86_words)
    } else {
        let next_sp = emu
            .sp()
            .checked_add(recipe.x86_words as u64 * size_of::<u32>() as u64)?;
        if next_sp > u32::MAX as u64 {
            return None;
        }
        (next_sp, recipe.x86_words)
    };
    (0..slots)
        .all(|i| call.arg(emu, i as usize).is_ok())
        .then_some(next_sp)
}

impl Simulation {
    fn outcome(self, is64: bool) -> (Option<u64>, Option<u64>) {
        let (result, last_error) = match self {
            Self::Void => return (None, None),
            // A declared unsupported-operation failure, before any output effects.
            Self::TimerFailure => (TIMERR_NOCANDO, None),
            Self::StatusFailure => (ERROR_CALL_NOT_IMPLEMENTED, None),
            Self::StatusSuccess => (0, None),
            Self::NtFailure => (STATUS_NOT_IMPLEMENTED, None),
            // Success status only: no output, object, or scheduler state is changed.
            Self::NtSuccess => (STATUS_SUCCESS, None),
            Self::KernelBoolean(value) => (u64::from(value), None),
            Self::Win32Failure => (FALSE, Some(ERROR_CALL_NOT_IMPLEMENTED)),
            Self::Win32Success => (TRUE, None),
            Self::MessageUnavailable => (u64::from(u32::MAX), Some(ERROR_CALL_NOT_IMPLEMENTED)),
            Self::HresultFailure => (E_NOTIMPL, None),
            Self::HresultSuccess => (S_OK, None),
            // Legacy best-effort return; output buffers and LastError stay unchanged.
            Self::GenericZero => (0, None),
            Self::WinsockFailure => (SOCKET_ERROR, Some(WSAEOPNOTSUPP)),
            Self::SocketFailure => (
                if is64 { INVALID_SOCKET } else { SOCKET_ERROR },
                Some(WSAEOPNOTSUPP),
            ),
            Self::WinsockSuccess => (0, None),
        };
        (Some(result), last_error)
    }
}

pub(crate) fn unsupported(emu: &mut Emu, library: &str, name: &str, address: u64) {
    unsupported_with_abi(emu, library, name, address, WinApi(Stdcall));
}

pub(crate) fn unsupported_with_abi(
    emu: &mut Emu,
    library: &str,
    name: &str,
    address: u64,
    call: WindowsAbi,
) {
    if emu.api_policy.stopped() {
        return;
    }
    let context = call_context(emu, library, name, address);
    let mode = emu.api_mode();
    let library = context.library.to_ascii_lowercase();
    let trace_inputs = log::log_enabled!(log::Level::Trace)
        && (!matches!(call, WinApi(_)) || emu.cfg.verbose >= 1);
    if mode == ApiMode::Strict {
        if emu
            .api_policy
            .mode
            .is_some_and(|mode| mode != ApiMode::Strict)
        {
            log::warn!("strict Windows API policy takes precedence over simulation settings");
        }
        if trace_inputs
            && let Some(signature) = recipe(
                &library,
                &context.name,
                ApiMode::SimulateFail,
                context.route,
            )
        {
            trace_signature(emu, call, &context, &signature, true);
        }
        stop(emu, context, ApiStopReason::StrictPolicy);
        return;
    }
    let signature = recipe(&library, &context.name, mode, context.route);
    let declared = signature.is_some();
    let recipe = signature
        .map(Ok)
        .unwrap_or_else(|| generic_recipe(emu, &context.name, call));
    let recipe = match recipe {
        Ok(recipe) => recipe,
        Err(reason) => {
            stop(emu, context, reason);
            return;
        }
    };
    if trace_inputs {
        trace_signature(emu, call, &context, &recipe, declared);
    }
    let Some(next_sp) = checked_frame(emu, &recipe, call) else {
        stop(emu, context, ApiStopReason::InvalidFrame);
        return;
    };
    let (result, last_error) = recipe.simulation.outcome(emu.cfg.arch.is_64bits());
    if let Some(error) = last_error {
        emu.set_last_error(error);
    }
    emu.set_sp(next_sp);
    if let Some(result) = result {
        call.set_return(emu, result);
    }
    emu.api_policy.simulation_count = emu.api_policy.simulation_count.saturating_add(1);
    log::warn!(
        "simulating {}!{}: {:?}, model={:?}, result={} LastError={last_error:?}",
        context.library,
        context.name,
        mode,
        recipe.simulation,
        result.map_or_else(|| "void".to_owned(), |value| format!("{value:#x}"))
    );
    emu.api_policy.last_outcome = Some(ApiOutcome {
        call: context,
        mode,
        disposition: if matches!(
            recipe.simulation,
            Simulation::GenericZero | Simulation::Void
        ) {
            ApiDisposition::SimulatedGeneric
        } else if mode == ApiMode::SimulateFail
            || matches!(recipe.simulation, Simulation::MessageUnavailable)
        {
            ApiDisposition::SimulatedFailure
        } else {
            ApiDisposition::SimulatedSuccess
        },
        result,
        last_error,
    });
}

/// Stop a unit-returning handler on a frame error. Fallible semantic helpers
/// propagate the error to this boundary instead of returning an API status.
macro_rules! api_try {
    ($emu:expr, $read:expr) => {
        match $read {
            Ok(value) => value,
            Err(error) => {
                $crate::api::windows::policy::argument_fault($emu, error);
                return;
            }
        }
    };
}

pub(crate) use api_try;
