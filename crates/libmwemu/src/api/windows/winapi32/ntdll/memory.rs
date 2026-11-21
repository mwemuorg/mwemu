use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::memory;
use crate::emu;
use crate::winapi::common::memory as memory_api;
use crate::winapi::common::process;
use crate::winapi::common::sections;
use crate::winapi::common::{crt, memory_ops};
use crate::winapi::policy;

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
        "memcpy" => memcpy(emu),
        "CheckRemoteDebuggerPresent" => CheckRemoteDebuggerPresent(emu),
        "NtFreeVirtualMemory" => NtFreeVirtualMemory(emu),
        "NtCreateSection" => NtCreateSection(emu),
        "NtMapViewOfSection" => NtMapViewOfSection(emu),
        "NtUnmapViewOfSection" => NtUnmapViewOfSection(emu),
        "NtFlushInstructionCache" => memory_api::nt_flush_cache(emu),
        "RtlAddFunctionTable" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlDeleteFunctionTable" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlInstallFunctionTableCallback" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "RtlLookupFunctionEntry" => policy::unsupported(emu, "ntdll", api, emu.pc()),
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

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    memory::nt_protect(emu, call);
}

fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    process::process_platform(emu, "ntdll", "CheckRemoteDebuggerPresent");
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
