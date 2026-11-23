pub mod advapi32;
pub(crate) mod bcrypt;
pub(crate) mod crypt32;
mod dnsapi;
pub(crate) mod gdi32;
mod iphlpapi;
pub mod kernel32;
mod kernelbase;
mod libgcc;
pub(crate) mod msvcrt;
pub mod ntdll;
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
use crate::api::windows::common::{crt, fallback};
use crate::api::windows::libraries;
use crate::api::windows::policy::{self, ApiRoute};
use crate::emu;
use crate::winapi::libraries::wincrt as ucrt;

pub fn gateway_by_import(emu: &mut emu::Emu, import_dll: &str, api: &str, addr: u32) {
    if emu.api_policy.stopped() {
        return;
    }
    let dll = import_dll.trim().to_ascii_lowercase();
    policy::enter_gateway(emu, &dll, api, addr as u64, ApiRoute::Import);
    if crt::is_universal_library(&dll) {
        ucrt::gateway(api, u64::from(addr), emu);
        policy::finish_call(emu);
        return;
    }
    if dll == "msvcrt.dll" {
        msvcrt::gateway_by_name(api, addr, emu);
        policy::finish_call(emu);
        return;
    }
    if libraries::dispatch(&dll, api, emu) {
        policy::finish_call(emu);
        return;
    }
    let resolved = kernel32::resolve_api_name_in_module(emu, &dll, api);
    if resolved != 0
        && let Some(map) = emu.maps.get_addr_name(resolved).map(str::to_string)
        && let Ok(resolved) = u32::try_from(resolved)
    {
        gateway(resolved, &map, emu);
        return;
    }
    match dll.trim_end_matches(".dll") {
        "kernel32" => {
            kernel32::gateway_by_name(addr, api, emu);
        }
        "kernelbase" => {
            kernelbase::gateway_by_name(addr, api, emu);
        }
        "ntdll" => {
            ntdll::gateway_by_name(addr, api, emu);
        }
        "libgcc_s_dw2-1" => {
            libgcc::gateway_by_name(addr, api, emu);
        }
        _ => policy::unsupported(emu, &dll, api, u64::from(addr)),
    }
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
        "vcruntime140.text" | "vcruntime140fothk" => libraries::gateway("vcruntime140", u64::from(addr), emu),
        "msvcrt.text" => msvcrt::gateway(addr, emu),
        "shell32.text" => shell32::gateway(addr, emu),
        "ntoskrnl.text" | "ntoskrnlPAGE" => libraries::gateway("ntoskrnl", addr as u64, emu),
        "mscoree.text" | "cryptbase.text" | "bcryptprimitives.text" | "shfolder.text" | "comctl32.text" | "gdiplus.text" | "uxtheme.text" | "version.text" => libraries::gateway(library, addr as u64, emu),
        "shlwapi.text" => shlwapi::gateway(addr, emu),
        "urlmon.text" => urlmon::gateway(addr, emu),
        "oleaut32.text" => oleaut32::gateway(addr, emu),
        "kernelbase.text" => kernelbase::gateway(addr, emu),
        "iphlpapi.text" => iphlpapi::gateway(addr, emu),
        "winmm.text" => winmm::gateway(addr, emu),
        "gdi32.text" => gdi32::gateway(addr, emu),
        "gdi32.rdata" => gdi32::gateway(addr, emu),
        "libgcc_s_dw2-1.text" => libgcc::gateway(addr, emu),
        section if crt::is_universal_library(library) && (section.ends_with(".text") || section.ends_with(".rdata")) => wincrt::gateway(addr, emu),
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
