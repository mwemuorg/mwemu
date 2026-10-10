use crate::api::windows::helper;
use crate::emu;
use crate::winapi::winapi64::kernel32::{
    self, EnterCriticalSection, InitializeCriticalSection, LeaveCriticalSection, TlsAlloc, TlsFree,
    TlsGetValue, TlsSetValue,
};
use crate::windows::constants;

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
        _ => return false,
    }
    true
}

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    emu.regs_mut().rax = 1;
}

fn RtlDeleteCriticalSection(emu: &mut emu::Emu) {
    let _cs = emu.regs().rcx;
    log_red!(emu, "ntdll!RtlDeleteCriticalSection");
    emu.regs_mut().rax = 0;
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let crit_sect_ptr = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    emu.regs_mut().rax = 1;
}

fn RtlQueueWorkItem(emu: &mut emu::Emu) {
    let fptr = emu.regs().rcx;
    let ctx = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!RtlQueueWorkItem  fptr: 0x{:x} ctx: 0x{:x} flags: {}",
        fptr,
        ctx,
        flags
    );

    if fptr > constants::LIBS_BARRIER64 {
        let name = kernel32::guess_api_name(emu, fptr);
        log::trace!("api: {} ", name);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtWaitForSingleObject(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let bAlert = emu.regs().rdx;
    let timeout = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!NtWaitForSingleObject  hndl: 0x{:x} timeout: {}",
        handle,
        timeout
    );

    emu.regs_mut().rax = 0x102;
}

fn RtlAddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx;
    let fptr = emu.regs().rdx;

    log_red!(
        emu,
        "ntdll!RtlAddVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(fptr);
    emu.regs_mut().rax = 0x2c2878;
}

fn RtlRemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx;
    let fptr = emu.regs().rdx;

    log_red!(
        emu,
        "ntdll!RtlRemoveVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(0);
    emu.regs_mut().rax = 0;
}

pub fn NtDelayExecution(emu: &mut emu::Emu) {
    let _alertable = emu.regs().rcx;
    let delay_interval_ptr = emu.regs().rdx;

    let interval = emu.maps.read_qword(delay_interval_ptr).unwrap_or(0) as i64;

    // Negative = relative time in 100ns units; convert to milliseconds
    let millis = if interval < 0 {
        ((-interval) / 10_000) as u64
    } else {
        (interval / 10_000) as u64
    };

    log_red!(emu, "ntdll!NtDelayExecution interval: {} ms", millis);

    helper::advance_tick(emu, millis);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
