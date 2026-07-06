use crate::context::context32::Context32;
use crate::debug::console::Console;
use crate::emu;
use crate::windows::constants;
use crate::windows::structures;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtGetContextThread" => NtGetContextThread(emu),
        "RtlVectoredExceptionHandler" => RtlVectoredExceptionHandler(emu),
        "RtlExitUserThread" => RtlExitUserThread(emu),
        "NtGetTickCount" => NtGetTickCount(emu),
        "NtQueryPerformanceCounter" => NtQueryPerformanceCounter(emu),
        "RtlGetVersion" => RtlGetVersion(emu),
        "RtlSetUnhandledExceptionFilter" => RtlSetUnhandledExceptionFilter(emu),
        "VerSetConditionMask" => VerSetConditionMask(emu),
        _ => return false,
    }
    true
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll_NtGetContextThread: error reading stack") as u64;
    let ctx_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll_NtGetContextThread: error reading context pointer") as u64;
    let ctx_ptr2 = emu
        .maps
        .read_dword(ctx_ptr)
        .expect("ntdll_NtGetContextThread: error reading context ptr") as u64;

    log_red!(emu, "ntdll_NtGetContextThread   ctx");

    let ctx = Context32::new(emu.regs());
    ctx.save(ctx_ptr2 as u32, &mut emu.maps);

    emu.regs_mut().rax = 0;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll_RtlVectoredExceptionHandler: error reading p1") as u64;
    let fptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll_RtlVectoredExceptionHandler: error reading fptr") as u64;

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(fptr);

    emu.regs_mut().rax = 0x2c2878;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

fn RtlExitUserThread(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlExitUserThread");
    Console::spawn_console(emu);
    emu.stop();
}

fn NtGetTickCount(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!NtGetTickCount");
    emu.regs_mut().rax = emu.tick as u64;
}

fn NtQueryPerformanceCounter(emu: &mut emu::Emu) {
    let perf_counter_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtQueryPerformanceCounter error reading perf_counter_ptr")
        as u64;
    let perf_freq_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 4)
            .expect("ntdll!NtQueryPerformanceCounter error reading perf_freq_ptr") as u64;

    log_red!(emu, "ntdll!NtQueryPerformanceCounter");

    emu.maps.write_dword(perf_counter_ptr, 0);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let versioninfo_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlLeaveCriticalSection error reading versioninfo_ptr param")
        as u64;

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new();
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlSetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let filter =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("ntdll!RtlSetUnhandledExceptionFilter error reading filter") as u64;

    log_red!(
        emu,
        "ntdll!RtlSetUnhandledExceptionFilter filter: 0x{:x}",
        filter
    );

    emu.set_uef(filter);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn VerSetConditionMask(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!VerSetConditionMask");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0xffff;
}
