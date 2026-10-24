use crate::context::context32::Context32;
use crate::debug::console::Console;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
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
        "NtQueryInformationProcess" => NtQueryInformationProcess(emu),
        "NtQuerySystemInformation" => NtQuerySystemInformation(emu),
        "NtSetInformationThread" => NtSetInformationThread(emu),
        "NtTerminateThread" => NtTerminateThread(emu),
        "RtlEncodePointer" => RtlEncodePointer(emu),
        "RtlDecodePointer" => RtlDecodePointer(emu),
        "RtlSizeHeap" => RtlSizeHeap(emu),
        _ => return false,
    }
    true
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let ctx_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let ctx_ptr2 = emu.maps.read_dword(ctx_ptr).unwrap_or(0) as u64;

    log_red!(emu, "ntdll_NtGetContextThread   ctx");

    let ctx = Context32::new(emu.regs());
    ctx.save(ctx_ptr2 as u32, &mut emu.maps);

    emu.regs_mut().rax = 0;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

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
    let perf_counter_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let perf_freq_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "ntdll!NtQueryPerformanceCounter");

    emu.maps.write_dword(perf_counter_ptr, 0);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let versioninfo_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new();
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlSetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let filter = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

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

fn NtQueryInformationProcess(emu: &mut emu::Emu) {
    let info_class = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    let buffer_length = arg(emu, 3) as u64;
    let return_length_ptr = arg(emu, 4) as u64;
    log_red!(
        emu,
        "ntdll!NtQueryInformationProcess class: {} buf: 0x{:x} len: {}",
        info_class,
        buffer,
        buffer_length
    );
    let fill = buffer_length.min(256);
    emu.maps.memset(buffer, 0, fill as usize);
    if info_class == 31 && buffer_length >= 4 {
        emu.maps.write_dword(buffer, 1); // ProcessDebugFlags
    }
    if return_length_ptr != 0 {
        emu.maps.write_dword(return_length_ptr, fill as u32);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 5);
}
fn NtQuerySystemInformation(emu: &mut emu::Emu) {
    let info_class = arg(emu, 0);
    let buffer = arg(emu, 1) as u64;
    let buffer_length = arg(emu, 2) as u64;
    let return_length_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "ntdll!NtQuerySystemInformation class: {} buf: 0x{:x} len: {}",
        info_class,
        buffer,
        buffer_length
    );
    let fill = buffer_length.min(256);
    emu.maps.memset(buffer, 0, fill as usize);
    if return_length_ptr != 0 {
        emu.maps.write_dword(return_length_ptr, fill as u32);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
fn NtSetInformationThread(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let info_class = arg(emu, 1);
    log_red!(
        emu,
        "ntdll!NtSetInformationThread handle: 0x{:x} class: {}",
        handle,
        info_class
    );
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
fn NtTerminateThread(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let status = arg(emu, 1);
    log_red!(emu, "ntdll!NtTerminateThread 0x{:x} {}", handle, status);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
fn RtlEncodePointer(emu: &mut emu::Emu) {
    let ptr = arg(emu, 0);
    log_red!(emu, "ntdll!RtlEncodePointer 0x{:x}", ptr);
    emu.regs_mut().rax = ptr as u64;
    stdcall_ret(emu, 1);
}
fn RtlDecodePointer(emu: &mut emu::Emu) {
    let ptr = arg(emu, 0);
    log_red!(emu, "ntdll!RtlDecodePointer 0x{:x}", ptr);
    emu.regs_mut().rax = ptr as u64;
    stdcall_ret(emu, 1);
}
fn RtlSizeHeap(emu: &mut emu::Emu) {
    let mem = arg(emu, 2) as u64;
    let size = crate::api::windows::common::heap::heap_allocation_size(emu, mem);
    log_red!(emu, "ntdll!RtlSizeHeap mem: 0x{:x} ={:?}", mem, size);
    emu.regs_mut().rax = size.map(|s| s as u64).unwrap_or(0xffff_ffff);
    stdcall_ret(emu, 3);
}
