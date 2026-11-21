use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu;
use crate::winapi::common::critical_section::critical_section;
use crate::winapi::common::threads;
use crate::winapi::libraries::kernel32::file_time::native_delay;
use crate::winapi::policy;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "RtlInitializeCriticalSection" => critical_section(emu, api),
        "RtlInitializeCriticalSectionAndSpinCount" => critical_section(emu, api),
        "RtlEnterCriticalSection" | "EnterCriticalSection" => critical_section(emu, api),
        "RtlLeaveCriticalSection" | "LeaveCriticalSection" => critical_section(emu, api),
        "RtlDeleteCriticalSection" | "DeleteCriticalSection" => critical_section(emu, api),
        "RtlInitializeCriticalSectionEx" => critical_section(emu, api),
        "RtlQueueWorkItem" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlAddVectoredExceptionHandler" => threads::vectored(emu, api, true),
        "RtlRemoveVectoredExceptionHandler" => threads::vectored(emu, api, false),
        // ntdll TLS workers — same slot semantics as the kernel32 wrappers.
        "RtlTlsAlloc" => threads::tls(emu, api),
        "RtlTlsFree" => threads::tls(emu, api),
        "RtlTlsGetValue" => threads::tls(emu, api),
        "RtlTlsSetValue" => threads::tls(emu, api),
        "NtDelayExecution" => NtDelayExecution(emu),
        "RtlInitializeSListHead" => RtlInitializeSListHead(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn RtlInitializeSListHead(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlInitializeSListHead head=0x{:x}", head);
    if head != 0 {
        emu.maps.write_qword(head, 0);
        emu.maps.write_qword(head + size_of::<u64>() as u64, 0);
    }
}

pub fn NtDelayExecution(emu: &mut emu::Emu) {
    native_delay(emu);
}
