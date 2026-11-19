use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::context::context32::Context32;
use crate::debug::console::Console;
use crate::emu;
use crate::winapi::common::ntdll;
use crate::winapi::common::ntdll::rtl_decompress_buffer;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::structures;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
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
        "RtlDecompressBuffer" => RtlDecompressBuffer(emu),
        "NtSetInformationThread" => NtSetInformationThread(emu),
        "NtTerminateThread" => NtTerminateThread(emu),
        "RtlEncodePointer" => RtlEncodePointer(emu),
        "RtlDecodePointer" => RtlDecodePointer(emu),
        "RtlSizeHeap" => RtlSizeHeap(emu),
        "RtlCaptureContext" => RtlCaptureContext(emu),
        "RestoreLastError" | "RtlRestoreLastWin32Error" => RtlRestoreLastWin32Error(emu),
        "NtQueueApcThread" => NtQueueApcThread(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn NtQueueApcThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let apc_routine = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(
        emu,
        "ntdll!NtQueueApcThread handle:0x{:x} routine:0x{:x}",
        handle,
        apc_routine
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 5);
}

fn RtlRestoreLastWin32Error(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlRestoreLastWin32Error err:{}", err);
    emu.set_last_error(err as u64);
    call.cleanup_args(emu, 1);
}

fn RtlCaptureContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ctx_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "ntdll!RtlCaptureContext ptr:0x{:x}", ctx_ptr);
    call.cleanup_args(emu, 1);
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let ctx_ptr = api_try!(emu, call.arg(emu, 1));
    let ctx_ptr2 = emu.maps.read_dword(ctx_ptr).unwrap_or(0) as u64;

    log_red!(emu, "ntdll_NtGetContextThread   ctx");

    let ctx = Context32::new(emu.regs());
    ctx.save(ctx_ptr2 as u32, &mut emu.maps);

    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

    call.set_return(emu, 0x2c2878);
    call.cleanup_args(emu, 2);
}

fn RtlExitUserThread(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlExitUserThread");
    Console::spawn_console(emu);
    emu.stop();
}

fn NtGetTickCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ntdll!NtGetTickCount");
    let return_value = emu.tick as u64;
    call.set_return(emu, return_value);
}

fn NtQueryPerformanceCounter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let perf_counter_ptr = api_try!(emu, call.arg(emu, 0));
    let perf_freq_ptr = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ntdll!NtQueryPerformanceCounter");

    emu.maps.write_dword(perf_counter_ptr, 0);

    call.cleanup_args(emu, 2);

    call.set_return(emu, STATUS_SUCCESS);
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let versioninfo_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new();
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

fn RtlSetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filter = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "ntdll!RtlSetUnhandledExceptionFilter filter: 0x{:x}",
        filter
    );

    emu.set_uef(filter);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

pub fn VerSetConditionMask(emu: &mut emu::Emu) {
    ntdll::version_condition_mask(emu);
}

fn NtQueryInformationProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_class = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let buffer_length = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let return_length_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
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
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 5);
}
fn NtQuerySystemInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_class = api_try!(emu, call.arg_u32(emu, 0));
    let buffer = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let buffer_length = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let return_length_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
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
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 4);
}
fn NtSetInformationThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let info_class = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "ntdll!NtSetInformationThread handle: 0x{:x} class: {}",
        handle,
        info_class
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 4);
}
fn NtTerminateThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let status = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "ntdll!NtTerminateThread 0x{:x} {}", handle, status);
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 2);
}
fn RtlEncodePointer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlEncodePointer 0x{:x}", ptr);
    call.set_return(emu, ptr as u64);
    call.cleanup_args(emu, 1);
}
fn RtlDecodePointer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlDecodePointer 0x{:x}", ptr);
    call.set_return(emu, ptr as u64);
    call.cleanup_args(emu, 1);
}
pub fn RtlDecompressBuffer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let compression_format = api_try!(emu, call.arg_u16(emu, 0));
    let uncompressed_buffer = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let uncompressed_buffer_size = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let compressed_buffer = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let compressed_buffer_size = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let final_uncompressed_size = api_try!(emu, call.arg_u32(emu, 5)) as u64;

    let return_value = rtl_decompress_buffer(
        emu,
        compression_format,
        uncompressed_buffer,
        uncompressed_buffer_size,
        compressed_buffer,
        compressed_buffer_size,
        final_uncompressed_size,
    );
    call.set_return(emu, return_value);
    call.cleanup_args(emu, 6);
}

fn RtlSizeHeap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let mem = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let size = crate::api::windows::common::heap::heap_allocation_size(emu, mem);
    log_red!(emu, "ntdll!RtlSizeHeap mem: 0x{:x} ={:?}", mem, size);
    call.set_return(emu, size.map(|s| s as u64).unwrap_or(0xffff_ffff));
    call.cleanup_args(emu, 3);
}
