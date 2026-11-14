use crate::api::windows::common::memory;
use crate::api::windows::common::memory::Call;
use crate::emu;
use crate::winapi::common::sections;
use crate::winapi::common::{crt, memory_ops};

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
        "RtlMoveMemory" => RtlMoveMemory(emu),
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "memset" => memset(emu),
        "RtlCopyMemory" => RtlCopyMemory(emu),
        "NtFlushInstructionCache" => NtFlushInstructionCache(emu),
        "RtlAddFunctionTable" => RtlAddFunctionTable(emu),
        "RtlDeleteFunctionTable" => RtlDeleteFunctionTable(emu),
        "RtlInstallFunctionTableCallback" => RtlInstallFunctionTableCallback(emu),
        "RtlCaptureContext" => RtlCaptureContext(emu),
        "RtlLookupFunctionEntry" => RtlLookupFunctionEntry(emu),
        "RtlVirtualUnwind" => RtlVirtualUnwind(emu),
        "NtFreeVirtualMemory" => NtFreeVirtualMemory(emu),
        "NtCreateSection" => NtCreateSection(emu),
        "NtMapViewOfSection" => NtMapViewOfSection(emu),
        "NtUnmapViewOfSection" => NtUnmapViewOfSection(emu),
        "CheckRemoteDebuggerPresent" => CheckRemoteDebuggerPresent(emu),
        "memcpy" => memcpy(emu),
        _ => return false,
    }
    true
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlZeroMemory", emu);
}

fn RtlMoveMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlMoveMemory", emu);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_allocate(emu, Call::Api, false);
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_query(emu, Call::Api);
}

/*
NTSTATUS NtProtectVirtualMemory(
  [in]      HANDLE  ProcessHandle,
  [in, out] PVOID   *BaseAddress,
  [in, out] PSIZE_T RegionSize,
  [in]      ULONG   NewProtect,
  [out]     PULONG  OldProtect
);
*/
fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_protect(emu, Call::Api);
}

fn memset(emu: &mut emu::Emu) {
    crt::fill(emu);
}

fn RtlCopyMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlCopyMemory", emu);
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let sz = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!NtFlushInstructionCache hndl: {:x} 0x{:x} sz: {}",
        proc_hndl,
        addr,
        sz
    );

    emu.regs_mut().rax = 0;
}

fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let function_table = emu.regs().rcx;
    let entry_count = emu.regs().rdx;
    let base_address = emu.regs().r8;

    log_red!(emu, "ntdll!RtlAddFunctionTable");

    emu.regs_mut().rax = 1;
}

/*
BOOLEAN RtlInstallFunctionTableCallback(
  [in] DWORD64                        TableIdentifier,
  [in] DWORD64                        BaseAddress,
  [in] DWORD                          Length,
  [in] PGET_RUNTIME_FUNCTION_CALLBACK Callback,
  [in] PVOID                          Context,
  [in] PCWSTR                         OutOfProcessCallbackDll
);
*/
fn RtlInstallFunctionTableCallback(emu: &mut emu::Emu) {
    let table_id = emu.regs().rcx;
    let base = emu.regs().rdx;
    let length = emu.regs().r8 & 0xffff_ffff;
    let callback = emu.regs().r9;

    log_red!(
        emu,
        "ntdll!RtlInstallFunctionTableCallback id: 0x{:x} base: 0x{:x} len: 0x{:x} callback: 0x{:x}",
        table_id,
        base,
        length,
        callback
    );

    // like RtlAddFunctionTable: accepted, not consulted by exception dispatch
    emu.regs_mut().rax = 1;
}

fn RtlDeleteFunctionTable(emu: &mut emu::Emu) {
    let function_table = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlDeleteFunctionTable table: 0x{:x}",
        function_table
    );

    emu.regs_mut().rax = 1;
}

fn RtlCaptureContext(emu: &mut emu::Emu) {
    let context_record = emu.regs().rcx as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlCaptureContext {:x}",
        emu.pos,
        context_record
    );
}

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let control_pc = emu.regs().rcx as usize;
    let image_base = emu.regs().rdx as usize;
    let history_table = emu.regs().r8 as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlLookupFunctionEntry {:x} {:x} {:x}",
        emu.pos,
        control_pc,
        image_base,
        history_table
    );
    emu.regs_mut().rax = 0;
}

fn RtlVirtualUnwind(emu: &mut emu::Emu) {
    let handler_type = emu.regs().rcx;
    let image_base = emu.regs().rdx;
    let control_pc = emu.regs().r8;
    let function_entry = emu.regs().r9;
    log_red!(
        emu,
        "** {} ntdll!RtlVirtualUnwind type:{} base:0x{:x} pc:0x{:x} entry:0x{:x}",
        emu.pos,
        handler_type,
        image_base,
        control_pc,
        function_entry
    );
    emu.regs_mut().rax = 0;
}

pub fn NtFreeVirtualMemory(emu: &mut emu::Emu) {
    memory::nt_free(emu, Call::Api);
}

pub fn NtCreateSection(emu: &mut emu::Emu) {
    sections::nt_create(emu, Call::Api);
}

pub fn NtMapViewOfSection(emu: &mut emu::Emu) {
    sections::nt_map(emu, Call::Api);
}

pub fn NtUnmapViewOfSection(emu: &mut emu::Emu) {
    sections::nt_unmap(emu, Call::Api);
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let _handle = emu.regs().rcx;
    let bool_ptr = emu.regs().rdx;

    log_red!(emu, "ntdll!CheckRemoteDebuggerPresent");

    if bool_ptr != 0 {
        emu.maps.write_dword(bool_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}

fn memcpy(emu: &mut emu::Emu) {
    crt::copy(emu);
}
