use crate::emu;
use crate::winapi::winapi64::kernel32::{
    self, InitializeCriticalSection, TlsAlloc, TlsFree, TlsGetValue, TlsSetValue,
};
use crate::windows::constants;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlInitializeCriticalSection" => InitializeCriticalSection(emu),
        "RtlInitializeCriticalSectionAndSpinCount" => RtlInitializeCriticalSectionAndSpinCount(emu),
        "RtlEnterCriticalSection" => RtlEnterCriticalSection(emu),
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

fn RtlEnterCriticalSection(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlEnterCriticalSection");

    emu.regs_mut().rax = 1;
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
