mod advapi32;
mod comctl32;
mod comctl64;
mod dnsapi;
mod gdi32;
pub mod kernel32;
mod kernelbase;
mod msvcrt;
mod ntdll;
mod ole32;
mod oleaut32;
mod shell32;
mod shlwapi;
mod urlmon;
mod user32;
mod uxtheme;
mod wincrt;
mod winhttp;
mod wininet;
mod ws2_32;

use crate::emu;

pub fn gateway_by_import(import_dll: &str, api: &str, emu: &mut emu::Emu) {
    let dll = import_dll.trim().to_lowercase();
    let api = api.trim();

    if dll.starts_with("api-ms-win-crt-") {
        let _ = wincrt::gateway_by_name(api, emu);
        emu.call_stack_mut().pop();
        return;
    }
    if dll == "msvcrt.dll" {
        let _ = msvcrt::gateway_by_name(api, emu);
        emu.call_stack_mut().pop();
        return;
    }

    // Fallback: if it isn't a CRT api-set, try to resolve it against loaded modules and
    // execute via the normal gateway mechanism.
    let resolved = kernel32::resolve_api_name_in_module(emu, &dll, api);
    if resolved != 0 {
        let map = emu.maps.get_addr_name(resolved).map(|s| s.to_string());
        if let Some(map) = map {
            gateway(resolved, &map, emu);
            return;
        }
    }

    log::warn!("unhandled import {}!{}", import_dll, api);
    emu.call_stack_mut().pop();
}

pub fn gateway(addr: u64, name: &str, emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_aarch64() {
        emu.regs_mut().sanitize64();
    }
    match name {
        "kernel32.text" => kernel32::gateway(addr, emu),
        "kernel32.rdata" => kernel32::gateway(addr, emu),
        "ntdll.text" => ntdll::gateway(addr, emu),
        "user32.text" => user32::gateway(addr, emu),
        "ws2_32.text" => ws2_32::gateway(addr, emu),
        "wininet.text" => wininet::gateway(addr, emu),
        "advapi32.text" => advapi32::gateway(addr, emu),
        "winhttp.text" => winhttp::gateway(addr, emu),
        "dnsapi.text" => dnsapi::gateway(addr, emu),
        "comctl32.text" => comctl32::gateway(addr, emu),
        "comctl64.text" => comctl64::gateway(addr, emu),
        "shell32.text" => shell32::gateway(addr, emu),
        "shlwapi.text" => shlwapi::gateway(addr, emu),
        "urlmon.text" => urlmon::gateway(addr, emu),
        "kernelbase.text" => kernelbase::gateway(addr, emu),
        "kernelbase.rdata" => kernelbase::gateway(addr, emu),
        "oleaut32.text" => oleaut32::gateway(addr, emu),
        "uxtheme.text" => uxtheme::gateway(addr, emu),
        "gdi32.text" => gdi32::gateway(addr, emu),
        "ole32.text" => ole32::gateway(addr, emu),
        "msvcrt.text" => msvcrt::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-stdio-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-heap-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "not_loaded" => {
            // TODO: banzai check?
            emu.pe64.as_ref().unwrap().import_addr_to_name(addr)
        }
        _ => panic!(
            "/!\\ winapi64 gateway: trying to execute on {} at 0x{:x}",
            name, addr
        ),
    };

    emu.call_stack_mut().pop();
}
