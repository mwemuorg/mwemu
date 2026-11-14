use crate::api::windows::helper;
use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::winapi::winapi64::kernel32::InitializeCriticalSection;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlInitializeCriticalSection" => InitializeCriticalSection(emu),
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

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = WinApiAbi::arg(emu, 0) as u32;
    let spin_count = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn RtlEnterCriticalSection(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);

    log_red!(emu, "ntdll!RtlEnterCriticalSection");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlLeaveCriticalSection(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);

    log_red!(emu, "ntdll!RtlLeaveCriticalSection");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlAddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = WinApiAbi::arg(emu, 0) as u32;
    let fptr = WinApiAbi::arg(emu, 1);

    log_red!(
        emu,
        "ntdll!RtlAddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.add_veh(first != 0, fptr);
    emu.regs_mut().rax = 0x2c2878;
}

fn RtlRemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);

    log_red!(
        emu,
        "ntdll!RtlRemoveVectoredExceptionHandler  handle: 0x{:x}",
        handle
    );

    emu.stack_pop32(false);

    emu.remove_veh(handle);
    emu.regs_mut().rax = 1;
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let crit_sect_ptr = WinApiAbi::arg(emu, 0);
    let spin_count = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn NtDelayExecution(emu: &mut emu::Emu) {
    let interval_ptr = arg(emu, 1) as u64;
    let interval = emu.maps.read_qword(interval_ptr).unwrap_or(0) as i64;
    // Negative = relative, in 100 ns units.
    let millis = interval.unsigned_abs() / 10_000;
    log_red!(emu, "ntdll!NtDelayExecution interval: {} ms", millis);
    helper::advance_tick(emu, millis);
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
fn NtWaitForSingleObject(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let timeout = arg(emu, 2);
    log_red!(
        emu,
        "ntdll!NtWaitForSingleObject hndl: 0x{:x} timeout: 0x{:x}",
        handle,
        timeout
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 3);
}
fn RtlQueueWorkItem(emu: &mut emu::Emu) {
    let fptr = arg(emu, 0);
    let ctx = arg(emu, 1);
    let flags = arg(emu, 2);
    log_red!(
        emu,
        "ntdll!RtlQueueWorkItem fptr: 0x{:x} ctx: 0x{:x} flags: {}",
        fptr,
        ctx,
        flags
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 3);
}

fn RtlDeleteCriticalSection(emu: &mut emu::Emu) {
    let _cs = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "ntdll!RtlDeleteCriticalSection");
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn RtlTlsAlloc(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlTlsAlloc");
    emu.tls32_mut().push(0);
    emu.regs_mut().rax = (emu.tls32().len() - 1) as u64;
}

fn RtlTlsFree(emu: &mut emu::Emu) {
    let index = WinApiAbi::arg(emu, 0) as usize;
    log_red!(emu, "ntdll!RtlTlsFree index: {}", index);
    emu.stack_pop32(false);
    if index < emu.tls32().len() {
        emu.tls32_mut()[index] = 0;
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn RtlTlsGetValue(emu: &mut emu::Emu) {
    let index = WinApiAbi::arg(emu, 0) as usize;
    emu.stack_pop32(false);
    let val = if index < emu.tls32().len() {
        emu.set_last_error(ERROR_SUCCESS);
        emu.tls32()[index] as u64
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };
    log_red!(emu, "ntdll!RtlTlsGetValue index: {} =0x{:x}", index, val);
    emu.regs_mut().rax = val;
}

fn RtlTlsSetValue(emu: &mut emu::Emu) {
    let index = WinApiAbi::arg(emu, 0) as usize;
    let value = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "ntdll!RtlTlsSetValue index: {} value: 0x{:x}",
        index,
        value
    );
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    if index < emu.tls32().len() {
        emu.tls32_mut()[index] = value;
    } else {
        while emu.tls32().len() <= index {
            emu.tls32_mut().push(0);
        }
        emu.tls32_mut()[index] = value;
    }
    emu.regs_mut().rax = 1;
}

fn RtlInitializeSListHead(emu: &mut emu::Emu) {
    let head = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ntdll!RtlInitializeSListHead head: 0x{:x}", head);
    if head != 0 {
        emu.maps.write_dword(head, 0);
        emu.maps.write_dword(head + 4, 0);
    }
    emu.stack_pop32(false);
}
