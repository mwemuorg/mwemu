pub(crate) mod advapi32;
pub(crate) mod bcrypt;
mod comctl32;
mod comctl64;
pub(crate) mod crypt32;
mod dnsapi;
pub(crate) mod gdi32;
pub mod kernel32;
mod kernelbase;
pub(crate) mod msvcrt;
pub mod ntdll;
pub(crate) mod ole32;
pub(crate) mod oleaut32;
pub(crate) mod shell32;
mod shlwapi;
mod urlmon;
pub(crate) mod user32;
mod uxtheme;
mod version;
mod wincrt;
pub(crate) mod winhttp;
pub(crate) mod wininet;
pub(crate) mod ws2_32;

use crate::emu;
use crate::winapi::helper;

pub fn gateway_by_import(emu: &mut emu::Emu, import_dll: &str, api: &str, addr: u64) {
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

    if emu.cfg.skip_unimplemented {
        log::warn!("banzai: skipping unhandled import {}!{}", import_dll, api);
        emu.regs_mut().rax = 0;
    } else {
        log::warn!("unhandled import {}!{}", import_dll, api);
    }
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
        "ole32.text" | "ole32.rdata" => ole32::gateway(addr, emu),
        "version.text" => version::gateway(addr, emu),
        "bcrypt.text" | "bcryptprimitives.text" => bcrypt::gateway(addr, emu),
        "winmm.text" => {
            let api = kernel32::guess_api_name(emu, addr);
            let api_name = api.split("!").last().unwrap_or(&api);
            match api_name {
                "timeBeginPeriod" | "timeEndPeriod" => {
                    let period = emu.regs().rcx as u32;
                    log_red!(emu, "winmm!{} period:{}", api_name, period);
                    helper::advance_tick(emu, 1);
                    emu.regs_mut().rax = 0;
                }
                "timeGetTime" => {
                    helper::advance_tick(emu, 1);
                    log_red!(emu, "winmm!timeGetTime => {}", emu.tick);
                    emu.regs_mut().rax = emu.tick as u64;
                }
                "PlaySoundW" => {
                    let sound_ptr = emu.regs().rcx;
                    let sound = if sound_ptr != 0 {
                        emu.maps.read_wide_string(sound_ptr)
                    } else {
                        String::new()
                    };
                    log_red!(emu, "winmm!PlaySoundW sound:'{}'", sound);
                    emu.regs_mut().rax = 1;
                }
                "mciSendStringA" | "mciSendStringW" => {
                    let cmd_ptr = emu.regs().rcx;
                    let cmd = if cmd_ptr != 0 {
                        emu.maps.read_string(cmd_ptr)
                    } else {
                        String::new()
                    };
                    log_red!(emu, "winmm!{} cmd:'{}'", api_name, cmd);
                    emu.regs_mut().rax = 0;
                }
                "waveInOpen" => {
                    let phwi = emu.regs().rcx;
                    log_red!(emu, "winmm!waveInOpen");
                    if phwi != 0 {
                        emu.maps.write_qword(phwi, 0x1337_0001);
                    }
                    emu.regs_mut().rax = 0;
                }
                "waveInClose"
                | "waveInStart"
                | "waveInStop"
                | "waveInAddBuffer"
                | "waveInPrepareHeader"
                | "waveInUnprepareHeader" => {
                    let handle = emu.regs().rcx;
                    log_red!(emu, "winmm!{} handle:0x{:x}", api_name, handle);
                    emu.regs_mut().rax = 0;
                }
                _ => {
                    log::warn!("winmm: unimplemented API {}", api_name);
                    emu.regs_mut().rax = 0;
                }
            }
            String::new()
        }
        "crypt32.text" => crypt32::gateway(addr, emu),
        "msvcrt.text" | "msvcrtfothk" => msvcrt::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-stdio-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-heap-l1-1-0.rdata" => wincrt::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "not_loaded" => {
            if emu.cfg.skip_unimplemented {
                let api = emu
                    .pe64
                    .as_ref()
                    .map(|pe| pe.import_addr_to_name(addr))
                    .unwrap_or_default();
                log::warn!(
                    "banzai: skipping not_loaded API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                String::new()
            } else {
                emu.pe64.as_ref().unwrap().import_addr_to_name(addr)
            }
        }
        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!("banzai: skipping unknown section {} at 0x{:x}", name, addr);
                emu.regs_mut().rax = 0;
            } else {
                log::error!(
                    "/!\\ winapi64 gateway: trying to execute on {} at 0x{:x}",
                    name,
                    addr
                );
            }
            return;
        }
    };

    emu.call_stack_mut().pop();
}
