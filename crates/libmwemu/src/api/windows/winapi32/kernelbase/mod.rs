use crate::api::dispatch_api;
use crate::api::windows::common::memory as memory_api;
use crate::api::windows::libraries::advapi32;
use crate::emu;
use crate::winapi::libraries::kernel32::dispatch_shared;
use crate::winapi::winapi32::kernel32;

use crate::winapi::common::security::GetTokenInformation;
mod local_alloc;
use local_alloc::LocalAlloc;
mod _initterm;
use _initterm::_initterm;
mod _initterm_e;
use _initterm_e::_initterm_e;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.rsplit('!').next().unwrap_or(&api);
    gateway_by_name(addr, api, emu)
}

pub(super) fn gateway_by_name(addr: u32, api: &str, emu: &mut emu::Emu) -> String {
    if dispatch_shared(api, "kernelbase", emu) {
        return String::new();
    }
    dispatch_api!(api, {
        "GetAce" => advapi32::GetAce(emu),
        "GetAclInformation" => advapi32::GetAclInformation(emu),
        "InitializeAcl" => advapi32::InitializeAcl(emu),
        "MapGenericMask" => advapi32::MapGenericMask(emu),
        "AreAllAccessesGranted" => advapi32::AreAllAccessesGranted(emu),
        "AreAnyAccessesGranted" => advapi32::AreAnyAccessesGranted(emu),
        "FlushInstructionCache" => memory_api::flush_instruction_cache(emu, "kernelbase!FlushInstructionCache"),

        "_initterm" => _initterm(emu),
        "_initterm_e" => _initterm_e(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
    },
        handled(()) => String::new(),
        fallback => kernel32::gateway_by_name(addr, api, emu),
    )
}
