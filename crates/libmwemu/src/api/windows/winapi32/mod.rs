pub mod advapi32;
pub(crate) mod bcrypt;
pub(crate) mod crypt32;
mod dnsapi;
pub(crate) mod gdi32;
mod gdiplus;
mod iphlpapi;
pub mod kernel32;
mod kernelbase;
mod libgcc;
mod mscoree;
pub(crate) mod msvcrt;
pub mod ntdll;
mod ntoskrnl;
pub(crate) mod ole32;
pub(crate) mod oleaut32;
pub(crate) mod shell32;
mod shlwapi;
mod urlmon;
pub(crate) mod user32;
mod wincrt;
pub(crate) mod winhttp;
pub(crate) mod wininet;
mod winmm;
pub(crate) mod ws2_32;

use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::api::windows::policy::{self, ApiRoute};
use crate::emu;

pub fn gateway_by_import(emu: &mut emu::Emu, import_dll: &str, api: &str, addr: u32) {
    if emu.api_policy.stopped() {
        return;
    }
    let dll = import_dll.trim().to_ascii_lowercase();
    policy::enter_gateway(emu, &dll, api, addr as u64, ApiRoute::Import);
    let resolved = kernel32::resolve_api_name_in_module(emu, &dll, api);
    if let Some(map) = emu.maps.get_addr_name(resolved).map(str::to_string)
        && let Ok(resolved) = u32::try_from(resolved)
    {
        gateway(resolved, &map, emu);
        return;
    }
    policy::unsupported(emu, &dll, api, addr as u64);
    policy::finish_call(emu);
}

pub fn gateway(addr: u32, name: &str, emu: &mut emu::Emu) {
    if emu.api_policy.stopped() {
        return;
    }
    let library = name.split('.').next().unwrap_or(name);
    policy::enter_gateway(emu, library, "", addr as u64, ApiRoute::Gateway);
    emu.regs_mut().sanitize32();
    dispatch_api!(name, {
        "kernel32.text" => kernel32::gateway(addr, emu),
        "kernel32.rdata" => kernel32::gateway(addr, emu),
        "ntdll.text" => ntdll::gateway(addr, emu),
        "user32.text" => user32::gateway(addr, emu),
        "ws2_32.text" => ws2_32::gateway(addr, emu),
        "wininet.text" => wininet::gateway(addr, emu),
        "advapi32.text" => advapi32::gateway(addr, emu),
        "bcrypt.text" => bcrypt::gateway(addr, emu),
        "ole32.text" => ole32::gateway(addr, emu),
        "winhttp.text" => winhttp::gateway(addr, emu),
        "crypt32.text" => crypt32::gateway(addr, emu),
        "dnsapi.text" => dnsapi::gateway(addr, emu),
        "mscoree.text" => mscoree::gateway(addr, emu),
        "msvcrt.text" => msvcrt::gateway(addr, emu),
        "shell32.text" => shell32::gateway(addr, emu),
        "shlwapi.text" => shlwapi::gateway(addr, emu),
        "urlmon.text" => urlmon::gateway(addr, emu),
        "oleaut32.text" => oleaut32::gateway(addr, emu),
        "kernelbase.text" => kernelbase::gateway(addr, emu),
        "iphlpapi.text" => iphlpapi::gateway(addr, emu),
        "winmm.text" => winmm::gateway(addr, emu),
        "gdi32.text" => gdi32::gateway(addr, emu),
        "gdi32.rdata" => gdi32::gateway(addr, emu),
        "msvcp60.text" => unhandled_library(addr, emu),
        "gdiplus.text" => gdiplus::gateway(addr, emu),
        "libgcc_s_dw2-1.text" => libgcc::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "ntoskrnl.text" => ntoskrnl::gateway(addr, emu),
        "ntoskrnlPAGE" => ntoskrnl::gateway(addr, emu),
        "not_loaded" => unloaded_import(addr, emu),
    },
        handled(result) => result,
        fallback => unhandled_library(addr, emu),
    );
    policy::finish_call(emu);
}

fn unloaded_import(addr: u32, emu: &mut emu::Emu) -> String {
    let api = emu
        .pe32
        .as_ref()
        .map(|pe| pe.import_addr_to_name(addr))
        .unwrap_or_default();
    fallback::unimplemented(addr, &api, emu)
}

fn unhandled_library(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    fallback::unimplemented(addr, &api, emu)
}
