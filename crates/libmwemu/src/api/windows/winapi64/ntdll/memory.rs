use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::memory;
use crate::emu;
use crate::winapi::common::memory as memory_api;
use crate::winapi::common::process;
use crate::winapi::common::sections;
use crate::winapi::common::threads;
use crate::winapi::common::{crt, memory_ops};
use crate::winapi::policy;
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
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "memset" => memset(emu),
        "NtFlushInstructionCache" => memory_api::nt_flush_cache(emu),
        "RtlAddFunctionTable" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlDeleteFunctionTable" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlInstallFunctionTableCallback" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlCaptureContext" => threads::capture_context(emu),
        "RtlLookupFunctionEntry" => RtlLookupFunctionEntry(emu),
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

fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let [pc, image_base, _history] = api_try!(emu, call.args(emu));
    let found = emu.runtime_function_entry(pc);
    log_red!(emu, "RtlLookupFunctionEntry pc=0x{:x}", pc);
    let result = found
        .and_then(|(base, entry)| emu.maps.write_qword(image_base, base).then_some(entry))
        .unwrap_or(0);
    call.set_return(emu, result);
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
    process::process_platform(emu, "ntdll", "CheckRemoteDebuggerPresent");
}

fn memcpy(emu: &mut emu::Emu) {
    crt::copy(emu);
}
