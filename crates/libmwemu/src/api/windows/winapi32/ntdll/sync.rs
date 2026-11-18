use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::helper;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlInitializeCriticalSection" => RtlInitializeCriticalSection(emu),
        "RtlInitializeCriticalSectionAndSpinCount" => RtlInitializeCriticalSectionAndSpinCount(emu),
        "RtlEnterCriticalSection" | "EnterCriticalSection" => RtlEnterCriticalSection(emu),
        "RtlLeaveCriticalSection" | "LeaveCriticalSection" => RtlLeaveCriticalSection(emu),
        "RtlDeleteCriticalSection" | "DeleteCriticalSection" => RtlDeleteCriticalSection(emu),
        "RtlInitializeCriticalSectionEx" => RtlInitializeCriticalSectionEx(emu),
        "RtlAddVectoredExceptionHandler" => RtlAddVectoredExceptionHandler(emu),
        "RtlRemoveVectoredExceptionHandler" => RtlRemoveVectoredExceptionHandler(emu),
        "NtDelayExecution" => NtDelayExecution(emu),
        "NtWaitForSingleObject" => NtWaitForSingleObject(emu),
        "RtlQueueWorkItem" => RtlQueueWorkItem(emu),
        "RtlTlsAlloc" => RtlTlsAlloc(emu),
        "RtlTlsFree" => RtlTlsFree(emu),
        "RtlTlsGetValue" => RtlTlsGetValue(emu),
        "RtlTlsSetValue" => RtlTlsSetValue(emu),
        "RtlInitializeSListHead" => RtlInitializeSListHead(emu),
        _ => return false,
    }
    true
}

fn RtlInitializeCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlInitializeCriticalSection ptr: 0x{:x}", crit_sect);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg_u32(emu, 0));
    let spin_count = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    call.cleanup_args(emu, 2);

    call.set_return(emu, 1);
}

fn RtlEnterCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "ntdll!RtlEnterCriticalSection");

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

fn RtlLeaveCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "ntdll!RtlLeaveCriticalSection");

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

fn RtlAddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg_u32(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlAddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    call.cleanup_args(emu, 2);

    emu.add_veh(first != 0, fptr);
    call.set_return(emu, 0x2c2878);
}

fn RtlRemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "ntdll!RtlRemoveVectoredExceptionHandler  handle: 0x{:x}",
        handle
    );

    call.cleanup_args(emu, 1);

    emu.remove_veh(handle);
    call.set_return(emu, 1);
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect_ptr = api_try!(emu, call.arg(emu, 0));
    let spin_count = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

fn NtDelayExecution(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let interval_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let interval = emu.maps.read_qword(interval_ptr).unwrap_or(0) as i64;
    // Negative = relative, in 100 ns units.
    let millis = interval.unsigned_abs() / 10_000;
    log_red!(emu, "ntdll!NtDelayExecution interval: {} ms", millis);
    helper::advance_tick(emu, millis);
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 2);
}
fn NtWaitForSingleObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let timeout = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "ntdll!NtWaitForSingleObject hndl: 0x{:x} timeout: 0x{:x}",
        handle,
        timeout
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 3);
}
fn RtlQueueWorkItem(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fptr = api_try!(emu, call.arg_u32(emu, 0));
    let ctx = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "ntdll!RtlQueueWorkItem fptr: 0x{:x} ctx: 0x{:x} flags: {}",
        fptr,
        ctx,
        flags
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 3);
}

fn RtlDeleteCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _cs = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlDeleteCriticalSection");
    call.cleanup_args(emu, 1);
    call.set_return(emu, 0);
}

fn RtlTlsAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ntdll!RtlTlsAlloc");
    emu.tls32_mut().push(0);
    let return_value = (emu.tls32().len() - 1) as u64;
    call.set_return(emu, return_value);
}

fn RtlTlsFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg(emu, 0)) as usize;
    log_red!(emu, "ntdll!RtlTlsFree index: {}", index);
    call.cleanup_args(emu, 1);
    if index < emu.tls32().len() {
        emu.tls32_mut()[index] = 0;
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
}

fn RtlTlsGetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg(emu, 0)) as usize;
    call.cleanup_args(emu, 1);
    let val = if index < emu.tls32().len() {
        emu.set_last_error(ERROR_SUCCESS);
        emu.tls32()[index] as u64
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };
    log_red!(emu, "ntdll!RtlTlsGetValue index: {} =0x{:x}", index, val);
    call.set_return(emu, val);
}

fn RtlTlsSetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg(emu, 0)) as usize;
    let value = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "ntdll!RtlTlsSetValue index: {} value: 0x{:x}",
        index,
        value
    );
    call.cleanup_args(emu, 2);
    if index < emu.tls32().len() {
        emu.tls32_mut()[index] = value;
    } else {
        while emu.tls32().len() <= index {
            emu.tls32_mut().push(0);
        }
        emu.tls32_mut()[index] = value;
    }
    call.set_return(emu, 1);
}

fn RtlInitializeSListHead(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlInitializeSListHead head: 0x{:x}", head);
    if head != 0 {
        emu.maps.write_dword(head, 0);
        emu.maps.write_dword(head + 4, 0);
    }
    call.cleanup_args(emu, 1);
}
