use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::winapi32::kernel32;

use crate::winapi::common::files::file_entry_points;

file_entry_points!(
    "kernel32",
    Wow64EnableWow64FsRedirection,
    _lclose,
    _lcreat,
    _llseek,
    _lopen,
    _lread,
    _lwrite,
);

unsupported_exports!(
    "kernel32",
    BaseDestroyVDMEnvironment,
    BaseUpdateVDMEntry,
    NtVdm64CreateProcessInternalW,
    RegisterConsoleVDM,
    VDMConsoleOperation,
    VDMOperationStarted,
    Wow64GetThreadSelectorEntry,
    Wow64SuspendThread,
);

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "_TlgCreateWsz" => kernel32::_TlgCreateWsz(emu),
        "_lclose" => kernel32::_lclose(emu),
        "_lcreat" => kernel32::_lcreat(emu),
        "_llseek" => kernel32::_llseek(emu),
        "_lopen" => kernel32::_lopen(emu),
        "_lread" => kernel32::_lread(emu),
        "_lwrite" => kernel32::_lwrite(emu),
        "_hread" => kernel32::_hread(emu),
        "_hwrite" => kernel32::_hwrite(emu),
        "__C_specific_handler" => kernel32::__C_specific_handler(emu),
        "__chkstk" => kernel32::__chkstk(emu),
        "__misaligned_access" => kernel32::__misaligned_access(emu),
        "_local_unwind" => kernel32::_local_unwind(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

unsupported_exports!(
    "kernel32",
    NtWow64ConsoleLaunchServerProcess,
    NtWow64CsrBaseCheckRunApp,
    NtWow64CsrBasepCreateActCtx,
    NtWow64CsrBasepCreateProcess,
    NtWow64CsrBasepDefineDosDevice,
    NtWow64CsrBasepGetProcessShutdownParam,
    NtWow64CsrBasepNlsGetUserInfo,
    NtWow64CsrBasepNlsUpdateCacheCount,
    NtWow64CsrBasepRefreshIniFileMapping,
    NtWow64CsrBasepSetClientTimeZoneInformation,
    NtWow64CsrBasepSetProcessShutdownParam,
    NtWow64CsrBasepSoundSentryNotification,
    Wow64SystemServiceCall,
    Wow64TransitionResolve,
    _TlgCreateWsz,
);
