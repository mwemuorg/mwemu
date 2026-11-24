use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::libraries::advapi32;
use crate::emu;
use crate::syscall::windows::syscall64::query_system_information;
use crate::winapi::common::ntdll;
pub use crate::winapi::common::ntdll::RtlDecompressBuffer;
use crate::winapi::common::threads;
use crate::winapi::libraries::kernel32::file_time::nt_performance;
use crate::winapi::policy;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if ntdll::etw(api, emu) {
        return true;
    }
    dispatch_api!(api, {
        "NtManageHotPatch" => ntdll::manage_hot_patch(emu, WinApi(Stdcall)),
        "RtlCreateAcl" => advapi32::RtlCreateAcl(emu),
        "RtlQueryInformationAcl" => advapi32::RtlQueryInformationAcl(emu),
        "RtlGetAce" => advapi32::RtlGetAce(emu),
        "RtlMapGenericMask" => advapi32::MapGenericMask(emu),
        "RtlAreAllAccessesGranted" => advapi32::AreAllAccessesGranted(emu),
        "RtlAreAnyAccessesGranted" => advapi32::AreAnyAccessesGranted(emu),
        "NtQueueApcThread" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "NtGetContextThread" => threads::nt_get_context_thread(emu),
        "RtlExitUserThread" => threads::exit_thread(emu),
        "NtGetTickCount" => ntdll::native_tick(emu),
        "NtQueryPerformanceCounter" => nt_performance(emu,WinApi(Stdcall)),
        "RtlGetVersion" => ntdll::version_info(emu, api),
        "RtlExitUserProcess" => threads::exit_process(emu, api),
        "RtlSetUnhandledExceptionFilter" => ntdll::set_exception_filter(emu),
        "NtTerminateThread" => threads::nt_terminate_thread(emu),
        // kernel32!RestoreLastError forwards to ntdll!RtlRestoreLastWin32Error:
        // just stores rcx into the last-error slot, no return value.
        "RtlRestoreLastWin32Error" | "RestoreLastError" => ntdll::restore_last_error(emu),
        "NtQuerySystemInformation" => NtQuerySystemInformation(emu),
        "RtlDecompressBuffer" => RtlDecompressBuffer(emu),
        "RtlEncodePointer" => ntdll::pointer_round_trip(emu, api),
        "RtlDecodePointer" => ntdll::pointer_round_trip(emu, api),
        "RtlVectoredExceptionHandler" => RtlVectoredExceptionHandler(emu),
        "VerSetConditionMask" => VerSetConditionMask(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

pub fn NtQuerySystemInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg_u32(emu, 0));
    let info = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg_u32(emu, 2));
    let ret_len = api_try!(emu, call.arg(emu, 3));
    query_system_information(emu, u64::from(class), info, len, ret_len);
    call.cleanup_args(emu, 4);
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler first={} callback=0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);
    call.set_return(emu, 0x2c2878);
}

pub fn VerSetConditionMask(emu: &mut emu::Emu) {
    ntdll::version_condition_mask(emu);
}
