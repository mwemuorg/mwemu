pub mod advapi32;
pub mod bcrypt;
pub mod comctl32;
pub mod crypt32;
pub mod dnsapi;
pub mod gdi32;
pub mod gdiplus;
pub mod http;
pub mod kernel32;
pub(crate) mod mscoree;
pub mod msvcrt;
pub mod ole32;
pub mod oleaut32;
pub mod shell32;
pub(crate) mod shfolder;
pub mod shlwapi;
pub(crate) mod unsupported;
pub mod urlmon;
pub mod user32;
pub mod uxtheme;
pub(crate) mod version;
pub mod wincrt;
pub mod winhttp;
pub mod wininet;
pub mod ws2_32;

use crate::api::windows::common::{crt, fallback, iphlpapi, ntdll, seh4};
use crate::emu::Emu;
use crate::winapi::{winapi32, winapi64};

pub(crate) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    match library.trim_end_matches(".dll") {
        "vcruntime140" => {
            seh4::dispatch(emu, "vcruntime140", api) || crt::dispatch_memory(api, emu)
        }
        "bcrypt" => bcrypt::dispatch(api, emu),
        "crypt32" => crypt32::dispatch(api, emu),
        "ole32" => ole32::dispatch(api, emu),
        "oleaut32" => oleaut32::dispatch(api, emu),
        "user32" => user32::dispatch(api, emu),
        "cryptbase" if api == "SystemFunction036" => {
            advapi32::system_random(emu, "cryptbase");
            true
        }
        "bcryptprimitives" if api == "ProcessPrng" => {
            bcrypt::ProcessPrng(emu);
            true
        }
        "advapi32" => advapi32::dispatch(api, emu),
        "winmm" => kernel32::system::dispatch_multimedia_timer(api, emu),
        "mscoree" if api == "_CorExeMain" => {
            mscoree::entry(emu);
            true
        }
        "ntoskrnl" => ntdll::dispatch_counted_string(api, "ntoskrnl", emu),
        "comctl32" => comctl32::dispatch(api, emu),
        "dnsapi" => dnsapi::dispatch(api, emu),
        "gdi32" => gdi32::dispatch(api, emu),
        "iphlpapi" => iphlpapi::dispatch(api, emu),
        "shell32" => shell32::dispatch(api, emu),
        "shlwapi" => shlwapi::dispatch(api, emu),
        "urlmon" => urlmon::dispatch(api, emu),
        "winhttp" => winhttp::dispatch(api, emu),
        "wininet" => wininet::dispatch(api, emu),
        "ws2_32" => ws2_32::dispatch(api, emu),
        "gdiplus" => gdiplus::dispatch(api, emu),
        "shfolder" => shfolder::dispatch(api, emu),
        "uxtheme" => uxtheme::dispatch(api, emu),
        "version" | "kernelbase" => version::dispatch(api, emu),
        _ => false,
    }
}

pub(crate) fn gateway(library: &str, address: u64, emu: &mut Emu) -> String {
    dispatch_address(address, emu, |api, emu| dispatch(library, api, emu))
}

pub(crate) fn dispatch_address(
    address: u64,
    emu: &mut Emu,
    dispatch: impl FnOnce(&str, &mut Emu) -> bool,
) -> String {
    let qualified = if emu.cfg.arch.is_64bits() {
        winapi64::kernel32::guess_api_name(emu, address)
    } else {
        winapi32::kernel32::guess_api_name(emu, address as u32)
    };
    let api = qualified.rsplit('!').next().unwrap_or(&qualified);
    if dispatch(api, emu) {
        String::new()
    } else {
        fallback::unimplemented(address, api, emu)
    }
}
