use crate::context::context64::Context64;
use crate::debug::console::Console;
use crate::emu;
use crate::winapi::winapi64::kernel32::set_last_error;
use crate::windows::{constants, structures};

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "ZwQueueApcThread" => ZwQueueApcThread(emu),
        "NtGetContextThread" => NtGetContextThread(emu),
        "RtlExitUserThread" => RtlExitUserThread(emu),
        "NtGetTickCount" => NtGetTickCount(emu),
        "NtQueryPerformanceCounter" => NtQueryPerformanceCounter(emu),
        "RtlGetVersion" => RtlGetVersion(emu),
        "RtlSetUnhandledExceptionFilter" => RtlSetUnhandledExceptionFilter(emu),
        "NtTerminateThread" => NtTerminateThread(emu),
        "NtSetInformationThread" => NtSetInformationThread(emu),
        // kernel32!RestoreLastError forwards to ntdll!RtlRestoreLastWin32Error:
        // just stores rcx into the last-error slot, no return value.
        "RtlRestoreLastWin32Error" | "RestoreLastError" => RestoreLastError(emu),
        _ => return false,
    }
    true
}

fn RestoreLastError(emu: &mut emu::Emu) {
    let err_code = emu.regs().rcx;
    log_red!(emu, "ntdll!RtlRestoreLastWin32Error err: {}", err_code);
    set_last_error(err_code);
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;
    let ctx_ptr2 = emu
        .maps
        .read_qword(ctx_ptr)
        .expect("ntdll_NtGetContextThread: error reading context ptr");

    log_red!(emu, "ntdll_NtGetContextThread   ctx:");

    let ctx = Context64::new(emu.regs());
    ctx.save(ctx_ptr2, &mut emu.maps);

    emu.regs_mut().rax = 0;
}

fn RtlExitUserThread(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlExitUserThread");
    Console::spawn_console(emu);
    emu.stop();
}

fn ZwQueueApcThread(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let apc_routine = emu.regs().rdx;
    let apc_ctx = emu.regs().r8;
    let arg1 = emu.regs().r9;
    let arg2 = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!ZwQueueApcThread cannot read arg2");

    log_red!(
        emu,
        "ntdll!ZwQueueApcThread hndl: {} routine: {} ctx: {} arg1: {} arg2: {}",
        thread_handle,
        apc_routine,
        apc_ctx,
        arg1,
        arg2
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtGetTickCount(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!NtGetTickCount");
    emu.regs_mut().rax = emu.tick as u64;
}

fn NtQueryPerformanceCounter(emu: &mut emu::Emu) {
    let perf_counter_ptr = emu.regs().rcx;
    let perf_freq_ptr = emu.regs().rdx;

    log_red!(emu, "ntdll!NtQueryPerformanceCounter");

    emu.maps.write_dword(perf_counter_ptr, 0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let versioninfo_ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new();
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

    emu.regs_mut().rax = 1;
}

fn RtlSetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let filter = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlSetUnhandledExceptionFilter filter: 0x{:x}",
        filter
    );

    emu.set_uef(filter);
    emu.regs_mut().rax = 1;
}

fn NtTerminateThread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let exit_status = emu.regs().rdx;

    log_red!(emu, "ntdll!NtTerminateThread {:x} {}", handle, exit_status);

    emu.regs_mut().rax = 0;
}

fn NtSetInformationThread(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let thread_info_class = emu.regs().rdx;
    let thread_info_ptr = emu.regs().r8;
    let thread_info_length = emu.regs().r9;

    log_red!(
        emu,
        "ntdll!NtSetInformationThread handle: 0x{:x} class: {} info_ptr: 0x{:x} length: {}",
        thread_handle,
        thread_info_class,
        thread_info_ptr,
        thread_info_length
    );

    emu.regs_mut().rax = 0x00000000;
}
