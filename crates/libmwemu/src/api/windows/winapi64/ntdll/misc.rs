use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::context::context64::Context64;
use crate::debug::console::Console;
use crate::emu;
use crate::winapi::common::ntdll;
use crate::winapi::common::ntdll::rtl_decompress_buffer;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::structures;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtQueueApcThread" => NtQueueApcThread(emu),
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
        "RtlRestoreLastWin32Error" | "RestoreLastError" => RtlRestoreLastWin32Error(emu),
        "NtQueryInformationProcess" => NtQueryInformationProcess(emu),
        "NtQuerySystemInformation" => NtQuerySystemInformation(emu),
        "RtlDecompressBuffer" => RtlDecompressBuffer(emu),
        "RtlEncodePointer" => RtlEncodePointer(emu),
        "RtlDecodePointer" => RtlDecodePointer(emu),
        "RtlVectoredExceptionHandler" => RtlVectoredExceptionHandler(emu),
        "VerSetConditionMask" => VerSetConditionMask(emu),
        _ => return false,
    }
    true
}

fn RtlRestoreLastWin32Error(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err_code = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlRestoreLastWin32Error err: {}", err_code);
    emu.set_last_error(err_code);
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let ctx_ptr = api_try!(emu, call.arg(emu, 1));
    let ctx_ptr2 = emu.maps.read_qword(ctx_ptr).unwrap_or(0);

    log_red!(emu, "ntdll_NtGetContextThread   ctx:");

    let ctx = Context64::new(emu.regs());
    ctx.save(ctx_ptr2, &mut emu.maps);

    call.set_return(emu, 0);
}

fn RtlExitUserThread(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlExitUserThread");
    Console::spawn_console(emu);
    emu.stop();
}

fn NtQueueApcThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let thread_handle = api_try!(emu, call.arg(emu, 0));
    let apc_routine = api_try!(emu, call.arg(emu, 1));
    let apc_ctx = api_try!(emu, call.arg(emu, 2));
    let arg1 = api_try!(emu, call.arg(emu, 3));
    let arg2 = api_try!(emu, call.arg(emu, 4));

    log_red!(
        emu,
        "ntdll!NtQueueApcThread hndl: {} routine: {} ctx: {} arg1: {} arg2: {}",
        thread_handle,
        apc_routine,
        apc_ctx,
        arg1,
        arg2
    );

    call.set_return(emu, STATUS_SUCCESS);
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
    call.set_return(emu, STATUS_SUCCESS);
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let versioninfo_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new();
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

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
    call.set_return(emu, 1);
}

fn NtTerminateThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let exit_status = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ntdll!NtTerminateThread {:x} {}", handle, exit_status);

    call.set_return(emu, 0);
}

fn NtSetInformationThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let thread_handle = api_try!(emu, call.arg(emu, 0));
    let thread_info_class = api_try!(emu, call.arg(emu, 1));
    let thread_info_ptr = api_try!(emu, call.arg(emu, 2));
    let thread_info_length = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "ntdll!NtSetInformationThread handle: 0x{:x} class: {} info_ptr: 0x{:x} length: {}",
        thread_handle,
        thread_info_class,
        thread_info_ptr,
        thread_info_length
    );

    call.set_return(emu, 0x00000000);
}

pub fn NtQueryInformationProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _process_handle = api_try!(emu, call.arg(emu, 0));
    let info_class = api_try!(emu, call.arg(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    let buffer_length = api_try!(emu, call.arg(emu, 3));
    let return_length_ptr = api_try!(emu, call.arg(emu, 4));

    log_red!(
        emu,
        "ntdll!NtQueryInformationProcess class: {} buf: 0x{:x} len: {}",
        info_class,
        buffer,
        buffer_length
    );

    match info_class {
        0 => {
            // ProcessBasicInformation — 48 bytes
            let size: u64 = 48;
            for i in 0..(size / 8) {
                emu.maps.write_qword(buffer + i * 8, 0);
            }
            if return_length_ptr != 0 {
                emu.maps.write_dword(return_length_ptr, size as u32);
            }
        }
        _ => {
            // Unknown class: zero-fill what we can
            let fill = std::cmp::min(buffer_length, 256);
            for i in 0..fill {
                emu.maps.write_byte(buffer + i, 0);
            }
            if return_length_ptr != 0 {
                emu.maps.write_dword(return_length_ptr, fill as u32);
            }
        }
    }

    call.set_return(emu, STATUS_SUCCESS);
}

pub fn NtQuerySystemInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_class = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let buffer_length = api_try!(emu, call.arg(emu, 2));
    let return_length_ptr = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "ntdll!NtQuerySystemInformation class: {} buf: 0x{:x} len: {}",
        info_class,
        buffer,
        buffer_length
    );

    let fill = std::cmp::min(buffer_length, 256);
    for i in 0..fill {
        emu.maps.write_byte(buffer + i, 0);
    }
    if return_length_ptr != 0 {
        emu.maps.write_dword(return_length_ptr, fill as u32);
    }

    call.set_return(emu, STATUS_SUCCESS);
}

pub fn RtlDecompressBuffer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let compression_format = api_try!(emu, call.arg_u16(emu, 0));
    let uncompressed_buffer = api_try!(emu, call.arg(emu, 1));
    let uncompressed_buffer_size = api_try!(emu, call.arg(emu, 2));
    let compressed_buffer = api_try!(emu, call.arg(emu, 3));
    let compressed_buffer_size = api_try!(emu, call.arg(emu, 4));
    let final_uncompressed_size = api_try!(emu, call.arg(emu, 5));

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
}

fn RtlEncodePointer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlEncodePointer 0x{:x}", ptr);
    call.set_return(emu, ptr);
}

fn RtlDecodePointer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlDecodePointer 0x{:x}", ptr);
    call.set_return(emu, ptr);
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);
    call.set_return(emu, 0x2c2878);
}

pub fn VerSetConditionMask(emu: &mut emu::Emu) {
    ntdll::version_condition_mask(emu);
}
