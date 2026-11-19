use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::memory;
use crate::emu;
use crate::winapi::common::sections;
use crate::winapi::common::{crt, memory_ops};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    let call = WinApi(Stdcall);
    if memory_ops::dispatch(api, emu) || crt::dispatch(api, emu) {
        return true;
    }
    dispatch_api!(api, {
        "NtReadVirtualMemory" => memory::nt_process_memory(emu, call, false),
        "NtWriteVirtualMemory" => memory::nt_process_memory(emu, call, true),
        "NtAllocateVirtualMemory" => NtAllocateVirtualMemory(emu),
        "NtAllocateVirtualMemoryEx" => memory::nt_allocate(emu, call, true),
        "NtQueryVirtualMemory" => NtQueryVirtualMemory(emu),
        "RtlZeroMemory" => RtlZeroMemory(emu),
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "memset" => memset(emu),
        "memcpy" => memcpy(emu),
        "CheckRemoteDebuggerPresent" => CheckRemoteDebuggerPresent(emu),
        "NtFreeVirtualMemory" => NtFreeVirtualMemory(emu),
        "NtCreateSection" => NtCreateSection(emu),
        "NtMapViewOfSection" => NtMapViewOfSection(emu),
        "NtUnmapViewOfSection" => NtUnmapViewOfSection(emu),
        "RtlMoveMemory" => RtlMoveMemory(emu),
        "RtlCopyMemory" => RtlCopyMemory(emu),
        "NtFlushInstructionCache" => NtFlushInstructionCache(emu),
        "RtlAddFunctionTable" => RtlAddFunctionTable(emu),
        "RtlDeleteFunctionTable" => RtlDeleteFunctionTable(emu),
        "RtlInstallFunctionTableCallback" => RtlInstallFunctionTableCallback(emu),
        "RtlLookupFunctionEntry" => RtlLookupFunctionEntry(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlZeroMemory", emu);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_allocate(emu, call, false);
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_query(emu, call);
}

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_protect(emu, call);
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    let bool_ptr = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ntdll!CheckRemoteDebuggerPresent");

    emu.maps.write_dword(bool_ptr, 0);
    call.cleanup_args(emu, 2);

    call.set_return(emu, 1);
}

fn memset(emu: &mut emu::Emu) {
    crt::fill(emu);
}

fn memcpy(emu: &mut emu::Emu) {
    crt::copy(emu);
}

fn NtFreeVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_free(emu, call);
}

fn NtCreateSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_create(emu, call);
}

fn NtMapViewOfSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_map(emu, call);
}

fn NtUnmapViewOfSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_unmap(emu, call);
}

fn RtlMoveMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlMoveMemory", emu);
}

fn RtlCopyMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlCopyMemory", emu);
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg_u32(emu, 1));
    let sz = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ntdll!NtFlushInstructionCache 0x{:x} sz: {}", addr, sz);
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 3);
}

fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _table = api_try!(emu, call.arg_u32(emu, 0));
    let _count = api_try!(emu, call.arg_u32(emu, 1));
    let _base = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ntdll!RtlAddFunctionTable");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}

fn RtlDeleteFunctionTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let table = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ntdll!RtlDeleteFunctionTable table:0x{:x}", table);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

fn RtlInstallFunctionTableCallback(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let table_id = api_try!(emu, call.arg_u32(emu, 0));
    let base = api_try!(emu, call.arg_u32(emu, 1));
    let length = api_try!(emu, call.arg_u32(emu, 4));
    let callback = api_try!(emu, call.arg_u32(emu, 5));
    log_red!(
        emu,
        "ntdll!RtlInstallFunctionTableCallback id:0x{:x} base:0x{:x} len:0x{:x} cb:0x{:x}",
        table_id,
        base,
        length,
        callback
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let control_pc = api_try!(emu, call.arg_u32(emu, 0));
    let image_base = api_try!(emu, call.arg_u32(emu, 1));
    let history = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "ntdll!RtlLookupFunctionEntry pc:0x{:x} base:0x{:x} hist:0x{:x}",
        control_pc,
        image_base,
        history
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
