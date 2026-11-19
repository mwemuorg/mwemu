use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::memory;
use crate::emu;
use crate::winapi::common::sections;
use crate::winapi::common::{crt, memory_ops};
use crate::winapi::policy::api_try;

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
    },
        handled(()) => true,
        fallback => false,
    )
}

fn RtlZeroMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlZeroMemory", emu);
}

fn RtlMoveMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlMoveMemory", emu);
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_allocate(emu, call, false);
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_query(emu, call);
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
    let call = WinApi(Stdcall);
    memory::nt_protect(emu, call);
}

fn memset(emu: &mut emu::Emu) {
    crt::fill(emu);
}

fn RtlCopyMemory(emu: &mut emu::Emu) {
    memory_ops::dispatch("RtlCopyMemory", emu);
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let proc_hndl = api_try!(emu, call.arg(emu, 0));
    let addr = api_try!(emu, call.arg(emu, 1));
    let sz = api_try!(emu, call.arg(emu, 2));

    log_red!(
        emu,
        "ntdll!NtFlushInstructionCache hndl: {:x} 0x{:x} sz: {}",
        proc_hndl,
        addr,
        sz
    );

    call.set_return(emu, 0);
}

fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let function_table = api_try!(emu, call.arg(emu, 0));
    let entry_count = api_try!(emu, call.arg(emu, 1));
    let base_address = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "ntdll!RtlAddFunctionTable");

    call.set_return(emu, 1);
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
    let call = WinApi(Stdcall);
    let table_id = api_try!(emu, call.arg(emu, 0));
    let base = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg(emu, 2)) & 0xffff_ffff;
    let callback = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "ntdll!RtlInstallFunctionTableCallback id: 0x{:x} base: 0x{:x} len: 0x{:x} callback: 0x{:x}",
        table_id,
        base,
        length,
        callback
    );

    // like RtlAddFunctionTable: accepted, not consulted by exception dispatch
    call.set_return(emu, 1);
}

fn RtlDeleteFunctionTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let function_table = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "ntdll!RtlDeleteFunctionTable table: 0x{:x}",
        function_table
    );

    call.set_return(emu, 1);
}

fn RtlCaptureContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let context_record = api_try!(emu, call.arg(emu, 0)) as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlCaptureContext {:x}",
        emu.pos,
        context_record
    );
}

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let control_pc = api_try!(emu, call.arg(emu, 0)) as usize;
    let image_base = api_try!(emu, call.arg(emu, 1)) as usize;
    let history_table = api_try!(emu, call.arg(emu, 2)) as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlLookupFunctionEntry {:x} {:x} {:x}",
        emu.pos,
        control_pc,
        image_base,
        history_table
    );
    call.set_return(emu, 0);
}

fn RtlVirtualUnwind(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handler_type = api_try!(emu, call.arg(emu, 0));
    let image_base = api_try!(emu, call.arg(emu, 1));
    let control_pc = api_try!(emu, call.arg(emu, 2));
    let function_entry = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "** {} ntdll!RtlVirtualUnwind type:{} base:0x{:x} pc:0x{:x} entry:0x{:x}",
        emu.pos,
        handler_type,
        image_base,
        control_pc,
        function_entry
    );
    call.set_return(emu, 0);
}

pub fn NtFreeVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_free(emu, call);
}

pub fn NtCreateSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_create(emu, call);
}

pub fn NtMapViewOfSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_map(emu, call);
}

pub fn NtUnmapViewOfSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    sections::nt_unmap(emu, call);
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _handle = api_try!(emu, call.arg(emu, 0));
    let bool_ptr = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ntdll!CheckRemoteDebuggerPresent");

    if bool_ptr != 0 {
        emu.maps.write_dword(bool_ptr, 0);
    }
    call.set_return(emu, 1);
}

fn memcpy(emu: &mut emu::Emu) {
    crt::copy(emu);
}
