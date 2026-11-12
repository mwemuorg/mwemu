use crate::api::windows::common::memory;
use crate::api::windows::common::memory::Call;
use crate::emu;
use crate::winapi::common::sections;
use crate::winapi::common::{crt, memory_ops};
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if memory_ops::dispatch(api, emu) || crt::dispatch(api, emu) {
        return true;
    }
    match api {
        "NtReadVirtualMemory" => memory::nt_process_memory(emu, Call::Api, false),
        "NtWriteVirtualMemory" => memory::nt_process_memory(emu, Call::Api, true),
        "NtAllocateVirtualMemory" => NtAllocateVirtualMemory(emu),
        "NtAllocateVirtualMemoryEx" => memory::nt_allocate(emu, Call::Api, true),
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
        _ => return false,
    }
    true
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlZeroMemory", emu);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_allocate(emu, Call::Api, false);
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_query(emu, Call::Api);
}

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_protect(emu, Call::Api);
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let bool_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "ntdll!CheckRemoteDebuggerPresent");

    emu.maps.write_dword(bool_ptr, 0);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn memset(emu: &mut emu::Emu) {
    crt::fill(emu);
}

fn memcpy(emu: &mut emu::Emu) {
    crt::copy(emu);
}

fn NtFreeVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_free(emu, Call::Api);
}

fn NtCreateSection(emu: &mut emu::Emu) {
    sections::nt_create(emu, Call::Api);
}

fn NtMapViewOfSection(emu: &mut emu::Emu) {
    sections::nt_map(emu, Call::Api);
}

fn NtUnmapViewOfSection(emu: &mut emu::Emu) {
    sections::nt_unmap(emu, Call::Api);
}

fn RtlMoveMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlMoveMemory", emu);
}

fn RtlCopyMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlCopyMemory", emu);
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let addr = arg(emu, 1);
    let sz = arg(emu, 2);
    log_red!(emu, "ntdll!NtFlushInstructionCache 0x{:x} sz: {}", addr, sz);
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 3);
}

fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let _table = arg(emu, 0);
    let _count = arg(emu, 1);
    let _base = arg(emu, 2);
    log_red!(emu, "ntdll!RtlAddFunctionTable");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn RtlDeleteFunctionTable(emu: &mut emu::Emu) {
    let table = arg(emu, 0);
    log_red!(emu, "ntdll!RtlDeleteFunctionTable table:0x{:x}", table);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn RtlInstallFunctionTableCallback(emu: &mut emu::Emu) {
    let table_id = arg(emu, 0);
    let base = arg(emu, 1);
    let length = WinApiAbi::arg(emu, 4) as u32;
    let callback = WinApiAbi::arg(emu, 5) as u32;
    log_red!(
        emu,
        "ntdll!RtlInstallFunctionTableCallback id:0x{:x} base:0x{:x} len:0x{:x} cb:0x{:x}",
        table_id,
        base,
        length,
        callback
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let control_pc = arg(emu, 0);
    let image_base = arg(emu, 1);
    let history = arg(emu, 2);
    log_red!(
        emu,
        "ntdll!RtlLookupFunctionEntry pc:0x{:x} base:0x{:x} hist:0x{:x}",
        control_pc,
        image_base,
        history
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
