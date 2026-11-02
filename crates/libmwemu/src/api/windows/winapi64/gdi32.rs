use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BitBlt" => BitBlt(emu),
        "CreateCompatibleBitmap" => CreateCompatibleBitmap(emu),
        "CreateCompatibleDC" => CreateCompatibleDC(emu),
        "CreateDIBSection" => CreateDIBSection(emu),
        "CreateFontA" => CreateFontA(emu),
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        "CreateFontIndirectW" => CreateFontIndirectW(emu),
        "CreateFontW" => CreateFontW(emu),
        "CreatePen" => CreatePen(emu),
        "CreateSolidBrush" => CreateSolidBrush(emu),
        "DeleteDC" => DeleteDC(emu),
        "DeleteObject" => DeleteObject(emu),
        "Ellipse" => GdiEllipse(emu),
        "ExtTextOutA" => ExtTextOutA(emu),
        "ExtTextOutW" => ExtTextOutW(emu),
        "GetDeviceCaps" => GetDeviceCaps(emu),
        "GetObjectA" => GetObjectA(emu),
        "GetObjectW" => GetObjectW(emu),
        "GetPixel" => GetPixelGdi(emu),
        "GetStockObject" => GetStockObject(emu),
        "GetTextExtentPoint32A" => GetTextExtentPoint32A(emu),
        "GetTextExtentPoint32W" => GetTextExtentPoint32W(emu),
        "GetTextMetricsA" => GetTextMetricsA(emu),
        "GetTextMetricsW" => GetTextMetricsW(emu),
        "LineTo" => LineTo(emu),
        "MoveToEx" => MoveToEx(emu),
        "Rectangle" => GdiRectangle(emu),
        "RestoreDC" => RestoreDC(emu),
        "SaveDC" => SaveDC(emu),
        "SelectObject" => SelectObject(emu),
        "SetBkColor" => SetBkColor(emu),
        "SetBkMode" => SetBkMode(emu),
        "SetPixel" => SetPixel(emu),
        "SetStretchBltMode" => SetStretchBltMode(emu),
        "SetTextColor" => SetTextColor(emu),
        "StretchBlt" => StretchBlt(emu),
        "TextOutA" => TextOutA(emu),
        "TextOutW" => TextOutW(emu),
        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }
    String::new()
}

fn CreateCompatibleDC(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 0x1337_dc01;
}

fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "gdi32!CreateCompatibleBitmap {}x{}",
        emu.regs().rdx,
        emu.regs().r8
    );
    emu.regs_mut().rax = 0x1337_b001;
}

fn SelectObject(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = emu.regs().rdx;
}

fn DeleteObject(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn DeleteDC(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn BitBlt(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!BitBlt");
    emu.regs_mut().rax = 1;
}

fn GetDeviceCaps(emu: &mut emu::Emu) {
    let index = emu.regs().rdx as i32;
    let result: u64 = match index {
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
    log_red!(emu, "gdi32!GetDeviceCaps index:{} = {}", index, result);
    emu.regs_mut().rax = result;
}

fn CreateFontA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontA");
    emu.regs_mut().rax = 0x1337_0f01;
}

fn CreateFontW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontW");
    emu.regs_mut().rax = 0x1337_0f01;
}

fn CreateFontIndirectA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontIndirectA");
    emu.regs_mut().rax = 0x1337_0f02;
}

fn CreateFontIndirectW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontIndirectW");
    emu.regs_mut().rax = 0x1337_0f02;
}

fn GetTextMetricsA(emu: &mut emu::Emu) {
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16);
        emu.maps.write_dword(tm + 4, 13);
        emu.maps.write_dword(tm + 8, 3);
        emu.maps.write_dword(tm + 12, 0);
        emu.maps.write_dword(tm + 16, 0);
        emu.maps.write_dword(tm + 20, 7);
        emu.maps.write_dword(tm + 24, 14);
    }
    log_red!(emu, "gdi32!GetTextMetricsA");
    emu.regs_mut().rax = 1;
}

fn GetTextMetricsW(emu: &mut emu::Emu) {
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16);
        emu.maps.write_dword(tm + 4, 13);
        emu.maps.write_dword(tm + 8, 3);
        emu.maps.write_dword(tm + 12, 0);
        emu.maps.write_dword(tm + 16, 0);
        emu.maps.write_dword(tm + 20, 7);
        emu.maps.write_dword(tm + 24, 14);
    }
    log_red!(emu, "gdi32!GetTextMetricsW");
    emu.regs_mut().rax = 1;
}

fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32);
        emu.maps.write_dword(size + 4, 16);
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32A");
    emu.regs_mut().rax = 1;
}

fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32);
        emu.maps.write_dword(size + 4, 16);
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32W");
    emu.regs_mut().rax = 1;
}

fn SetBkMode(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetBkMode mode:{}", emu.regs().rdx);
    emu.regs_mut().rax = 1;
}

fn SetBkColor(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetBkColor color:0x{:x}", emu.regs().rdx);
    emu.regs_mut().rax = 0x00ff_ffff;
}

fn SetTextColor(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetTextColor color:0x{:x}", emu.regs().rdx);
    emu.regs_mut().rax = 0;
}

fn GetStockObject(emu: &mut emu::Emu) {
    let obj = emu.regs().rcx;
    log_red!(emu, "gdi32!GetStockObject index:{}", obj);
    emu.regs_mut().rax = 0x1337_0000 + obj;
}

fn CreateSolidBrush(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 0x1337_0b01;
}

fn CreatePen(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreatePen");
    emu.regs_mut().rax = 0x1337_0001;
}

fn MoveToEx(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!MoveToEx");
    emu.regs_mut().rax = 1;
}

fn LineTo(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!LineTo");
    emu.regs_mut().rax = 1;
}

fn GdiRectangle(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Rectangle");
    emu.regs_mut().rax = 1;
}

fn GdiEllipse(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Ellipse");
    emu.regs_mut().rax = 1;
}

fn SetPixel(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetPixel");
    emu.regs_mut().rax = 0;
}

fn GetPixelGdi(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetPixel");
    emu.regs_mut().rax = 0;
}

fn TextOutA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(emu.regs().r9);
    log_red!(emu, "gdi32!TextOutA '{}'", text);
    emu.regs_mut().rax = 1;
}

fn TextOutW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(emu.regs().r9);
    log_red!(emu, "gdi32!TextOutW '{}'", text);
    emu.regs_mut().rax = 1;
}

fn ExtTextOutA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!ExtTextOutA");
    emu.regs_mut().rax = 1;
}

fn ExtTextOutW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!ExtTextOutW");
    emu.regs_mut().rax = 1;
}

fn GetObjectA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetObjectA");
    emu.regs_mut().rax = 0;
}

fn GetObjectW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetObjectW");
    emu.regs_mut().rax = 0;
}

fn CreateDIBSection(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateDIBSection");
    emu.regs_mut().rax = 0x1337_0001;
}

fn StretchBlt(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!StretchBlt");
    emu.regs_mut().rax = 1;
}

fn SetStretchBltMode(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetStretchBltMode");
    emu.regs_mut().rax = 1;
}

fn SaveDC(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SaveDC");
    emu.regs_mut().rax = 1;
}

fn RestoreDC(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!RestoreDC");
    emu.regs_mut().rax = 1;
}
