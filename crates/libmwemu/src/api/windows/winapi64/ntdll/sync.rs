use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::helper;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::kernel32::{
    self, EnterCriticalSection, InitializeCriticalSection, LeaveCriticalSection, TlsAlloc, TlsFree,
    TlsGetValue, TlsSetValue,
};
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlInitializeCriticalSection" => InitializeCriticalSection(emu),
        "RtlInitializeCriticalSectionAndSpinCount" => RtlInitializeCriticalSectionAndSpinCount(emu),
        "RtlEnterCriticalSection" | "EnterCriticalSection" => EnterCriticalSection(emu),
        "RtlLeaveCriticalSection" | "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "RtlDeleteCriticalSection" | "DeleteCriticalSection" => RtlDeleteCriticalSection(emu),
        "RtlInitializeCriticalSectionEx" => RtlInitializeCriticalSectionEx(emu),
        "RtlQueueWorkItem" => RtlQueueWorkItem(emu),
        "NtWaitForSingleObject" => NtWaitForSingleObject(emu),
        "RtlAddVectoredExceptionHandler" => RtlAddVectoredExceptionHandler(emu),
        "RtlRemoveVectoredExceptionHandler" => RtlRemoveVectoredExceptionHandler(emu),
        // ntdll TLS workers — same slot semantics as the kernel32 wrappers.
        "RtlTlsAlloc" => TlsAlloc(emu),
        "RtlTlsFree" => TlsFree(emu),
        "RtlTlsGetValue" => TlsGetValue(emu),
        "RtlTlsSetValue" => TlsSetValue(emu),
        "NtDelayExecution" => NtDelayExecution(emu),
        "RtlInitializeSListHead" => RtlInitializeSListHead(emu),
        _ => return false,
    }
    true
}

fn RtlInitializeSListHead(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlInitializeSListHead head: 0x{:x}", head);
    if head != 0 {
        emu.maps.write_qword(head, 0);
        emu.maps.write_qword(head + size_of::<u64>() as u64, 0);
    }
}

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg(emu, 0));
    let spin_count = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    call.set_return(emu, 1);
}

fn RtlDeleteCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _cs = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlDeleteCriticalSection");
    call.set_return(emu, 0);
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect_ptr = api_try!(emu, call.arg(emu, 0));
    let spin_count = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    call.set_return(emu, 1);
}

fn RtlQueueWorkItem(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fptr = api_try!(emu, call.arg(emu, 0));
    let ctx = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));

    log_red!(
        emu,
        "ntdll!RtlQueueWorkItem  fptr: 0x{:x} ctx: 0x{:x} flags: {}",
        fptr,
        ctx,
        flags
    );

    if fptr > LIBS_BARRIER64 {
        let name = kernel32::guess_api_name(emu, fptr);
        log::trace!("api: {} ", name);
    }

    call.set_return(emu, STATUS_SUCCESS);
}

fn NtWaitForSingleObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let bAlert = api_try!(emu, call.arg(emu, 1));
    let timeout = api_try!(emu, call.arg(emu, 2));

    log_red!(
        emu,
        "ntdll!NtWaitForSingleObject  hndl: 0x{:x} timeout: {}",
        handle,
        timeout
    );

    call.set_return(emu, 0x102);
}

fn RtlAddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlAddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);
    // the handler address doubles as the handle RtlRemoveVectoredExceptionHandler gets
    call.set_return(emu, fptr);
}

fn RtlRemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "ntdll!RtlRemoveVectoredExceptionHandler  handle=0x{:x}",
        handle
    );

    emu.remove_veh(handle);
    call.set_return(emu, 1);
}

pub fn NtDelayExecution(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _alertable = api_try!(emu, call.arg(emu, 0));
    let delay_interval_ptr = api_try!(emu, call.arg(emu, 1));

    let interval = emu.maps.read_qword(delay_interval_ptr).unwrap_or(0) as i64;

    // Negative = relative time in 100ns units; convert to milliseconds
    let millis = if interval < 0 {
        ((-interval) / 10_000) as u64
    } else {
        (interval / 10_000) as u64
    };

    log_red!(emu, "ntdll!NtDelayExecution interval: {} ms", millis);

    helper::advance_tick(emu, millis);

    call.set_return(emu, STATUS_SUCCESS);
}
