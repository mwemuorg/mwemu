pub mod advapi32;
mod bcrypt;
mod crypt32;
mod dnsapi;
mod iphlpapi;
pub mod kernel32;
mod kernelbase;
mod libgcc;
mod mscoree;
mod msvcrt;
pub mod ntdll;
mod ntoskrnl;
mod ole32;
mod oleaut32;
mod shell32;
mod shlwapi;
mod urlmon;
mod user32;
mod wincrt;
mod winhttp;
mod wininet;
mod ws2_32;

use crate::emu;

/// The `i`-th (0-based) stdcall argument of the API being emulated. The
/// gateway already consumed the return address, so arguments start at `esp`.
pub(crate) fn arg(emu: &emu::Emu, i: u64) -> u32 {
    emu.maps
        .read_dword(emu.regs().get_esp() + i * 4)
        .unwrap_or(0)
}

/// stdcall: the callee removes its `n` arguments from the stack.
pub(crate) fn stdcall_ret(emu: &mut emu::Emu, n: u32) {
    for _ in 0..n {
        emu.stack_pop32(false);
    }
}

pub fn gateway(addr: u32, name: &str, emu: &mut emu::Emu) {
    emu.regs_mut().sanitize32();
    match name {
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
        "winmm.text" => {
            let api = kernel32::guess_api_name(emu, addr);
            let api_name = api.split("!").last().unwrap_or(&api);
            match api_name {
                "timeBeginPeriod"
                | "timeEndPeriod"
                | "waveInClose"
                | "waveInStart"
                | "waveInStop"
                | "waveInAddBuffer"
                | "waveInPrepareHeader"
                | "waveInUnprepareHeader"
                | "mciSendStringA"
                | "mciSendStringW" => {
                    log_red!(emu, "winmm!{}", api_name);
                    emu.regs_mut().rax = 0;
                }
                "timeGetTime" => {
                    log_red!(emu, "winmm!timeGetTime");
                    emu.regs_mut().rax = emu.tick as u64;
                }
                "PlaySoundW" => {
                    log_red!(emu, "winmm!PlaySoundW");
                    emu.regs_mut().rax = 1;
                }
                "waveInOpen" => {
                    let phwi = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
                    log_red!(emu, "winmm!waveInOpen");
                    if phwi != 0 {
                        emu.maps.write_dword(phwi, 0x1337_0001);
                    }
                    emu.regs_mut().rax = 0;
                }
                _ => {
                    log::warn!("winmm: unimplemented API {}", api_name);
                }
            }
            String::new()
        }
        "gdi32.text" => {
            let api = kernel32::guess_api_name(emu, addr);
            let api_name = api.split("!").last().unwrap_or(&api);
            match api_name {
                "CreateDCA" | "CreateCompatibleDC" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 0xdc_0001;
                }
                "DeleteDC" | "DeleteObject" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "GetDeviceCaps" => {
                    let cap = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
                    log_red!(emu, "gdi32!GetDeviceCaps cap:{}", cap);
                    emu.regs_mut().rax = match cap {
                        8 => 1920,   // HORZRES
                        10 => 1080,  // VERTRES
                        12 => 96,    // LOGPIXELSX
                        14 => 32,    // BITSPIXEL
                        88 => 96,    // LOGPIXELSX
                        90 => 96,    // LOGPIXELSY
                        117 => 1080, // DESKTOPVERTRES
                        118 => 1920, // DESKTOPHORZRES
                        _ => 0,
                    };
                }
                "CreateCompatibleBitmap" => {
                    log_red!(emu, "gdi32!CreateCompatibleBitmap");
                    emu.regs_mut().rax = 0xb0_0001;
                }
                "SelectObject" => {
                    let obj = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
                    log_red!(emu, "gdi32!SelectObject obj:0x{:x}", obj);
                    emu.regs_mut().rax = obj;
                }
                "GetObjectA" | "GetObjectW" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 0;
                }
                "StretchBlt" | "BitBlt" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "GetDIBits" => {
                    log_red!(emu, "gdi32!GetDIBits");
                    emu.regs_mut().rax = 0;
                }
                "CreateFontA" | "CreateFontW" | "CreateFontIndirectA" | "CreateFontIndirectW" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 0x1337_f001;
                }
                "GetTextMetricsA" | "GetTextMetricsW" => {
                    let tm = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
                    if tm != 0 {
                        emu.maps.write_dword(tm, 16);
                        emu.maps.write_dword(tm + 4, 13);
                        emu.maps.write_dword(tm + 8, 3);
                        emu.maps.write_dword(tm + 12, 0);
                        emu.maps.write_dword(tm + 16, 0);
                        emu.maps.write_dword(tm + 20, 7);
                        emu.maps.write_dword(tm + 24, 14);
                    }
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "GetTextExtentPoint32A" | "GetTextExtentPoint32W" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "SetBkMode" | "SetBkColor" | "SetTextColor" | "SetStretchBltMode" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "GetStockObject" => {
                    let idx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
                    log_red!(emu, "gdi32!GetStockObject index:{}", idx);
                    emu.regs_mut().rax = 0x1337_0000 + idx;
                }
                "CreateSolidBrush" => {
                    log_red!(emu, "gdi32!CreateSolidBrush");
                    emu.regs_mut().rax = 0x1337_0001;
                }
                "CreatePen" => {
                    log_red!(emu, "gdi32!CreatePen");
                    emu.regs_mut().rax = 0x1337_0001;
                }
                "CreateDIBSection" => {
                    log_red!(emu, "gdi32!CreateDIBSection");
                    emu.regs_mut().rax = 0x1337_0001;
                }
                "MoveToEx" | "LineTo" | "Rectangle" | "Ellipse" | "SaveDC" | "RestoreDC" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                "SetPixel" | "GetPixel" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 0;
                }
                "TextOutA" | "TextOutW" | "ExtTextOutA" | "ExtTextOutW" => {
                    log_red!(emu, "gdi32!{}", api_name);
                    emu.regs_mut().rax = 1;
                }
                _ => {
                    if emu.cfg.skip_unimplemented {
                        log::warn!("banzai: gdi32 skipping {}", api_name);
                        emu.regs_mut().rax = 0;
                    } else {
                        log::warn!("gdi32: unimplemented API {}", api_name);
                    }
                }
            }
            String::new()
        }
        "msvcp60.text" => {
            let api = kernel32::guess_api_name(emu, addr);
            let api_name = api.split("!").last().unwrap_or(&api);
            log_red!(emu, "msvcp60!{}", api_name);
            emu.regs_mut().rax = 0;
            String::new()
        }
        "gdiplus.text" => {
            let api = kernel32::guess_api_name(emu, addr);
            let api_name = api.split("!").last().unwrap_or(&api);
            match api_name {
                "GdiplusStartup" => {
                    log_red!(emu, "gdiplus!GdiplusStartup");
                    emu.regs_mut().rax = 0;
                }
                "GdipAlloc" => {
                    let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(256) as u64;
                    let alloc_addr = if let Some(a) = emu.maps.alloc(size) {
                        let _ = emu.maps.create_map(
                            &format!("gdip_alloc_{:x}", a),
                            a,
                            size,
                            crate::maps::mem64::Permission::READ_WRITE,
                        );
                        a
                    } else {
                        0
                    };
                    log_red!(emu, "gdiplus!GdipAlloc sz:{} => 0x{:x}", size, alloc_addr);
                    emu.regs_mut().rax = alloc_addr;
                }
                "GdipFree" => {
                    log_red!(emu, "gdiplus!GdipFree");
                    emu.regs_mut().rax = 0;
                }
                _ => {
                    log_red!(emu, "gdiplus!{}", api_name);
                    emu.regs_mut().rax = 0;
                }
            }
            String::new()
        }
        "libgcc_s_dw2-1.text" => libgcc::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "ntoskrnl.text" => ntoskrnl::gateway(addr, emu),
        "ntoskrnlPAGE" => ntoskrnl::gateway(addr, emu),
        "not_loaded" => {
            if emu.cfg.skip_unimplemented {
                let api = emu
                    .pe32
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
                emu.pe32.as_ref().unwrap().import_addr_to_name(addr)
            }
        }
        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!("banzai: skipping unknown section {} at 0x{:x}", name, addr);
                emu.regs_mut().rax = 0;
            } else {
                log::error!(
                    "/!\\ winapi32 gateway: trying to execute on {} at 0x{:x}",
                    name,
                    addr
                );
            }
            return;
        }
    };
    emu.call_stack_mut().pop();
}
