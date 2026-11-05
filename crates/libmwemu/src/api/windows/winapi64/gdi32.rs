use crate::emu;
use crate::serialization;
use crate::winapi::helper;
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
        "GetBkColor" => GetBkColor(emu),
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
        "AddFontResourceExA" => AddFontResourceExA(emu),
        "AddFontResourceExW" => AddFontResourceExW(emu),
        "Arc" => GdiArc(emu),
        "Chord" => GdiChord(emu),
        "CombineRgn" => CombineRgn(emu),
        "CreateBrushIndirect" => CreateBrushIndirect(emu),
        "CreateDCA" => CreateDCA(emu),
        "CreateDCW" => CreateDCW(emu),
        "CreateHatchBrush" => CreateHatchBrush(emu),
        "CreatePatternBrush" => CreatePatternBrush(emu),
        "CreatePenIndirect" => CreatePenIndirect(emu),
        "CreateRectRgn" => CreateRectRgn(emu),
        "CreateRectRgnIndirect" => CreateRectRgnIndirect(emu),
        "EndDoc" => EndDoc(emu),
        "EndPage" => EndPage(emu),
        "EnumFontFamiliesExA" => EnumFontFamiliesExA(emu),
        "EnumFontFamiliesExW" => EnumFontFamiliesExW(emu),
        "ExcludeClipRect" => ExcludeClipRect(emu),
        "FillRect" => GdiFillRect(emu),
        "GetCharABCWidthsA" => GetCharABCWidthsA(emu),
        "GetCharABCWidthsW" => GetCharABCWidthsW(emu),
        "GetCharWidth32A" => GetCharWidth32A(emu),
        "GetCharWidth32W" => GetCharWidth32W(emu),
        "GetClipBox" => GetClipBox(emu),
        "GetCurrentObject" => GetCurrentObject(emu),
        "GetDC" => GetDC(emu),
        "GetDIBColorTable" => GetDIBColorTable(emu),
        "GetDIBits" => GetDIBits(emu),
        "GetLayout" => GetLayout(emu),
        "GetMapMode" => GetMapMode(emu),
        "GetNearestColor" => GetNearestColor(emu),
        "GetObjectType" => GetObjectType(emu),
        "GetPolyFillMode" => GetPolyFillMode(emu),
        "GetROP2" => GetROP2(emu),
        "GetStretchBltMode" => GetStretchBltModeGdi(emu),
        "GetSystemPaletteEntries" => GetSystemPaletteEntries(emu),
        "GetTextAlign" => GetTextAlign(emu),
        "GetTextColor" => GetTextColor(emu),
        "GetTextFaceA" => GetTextFaceA(emu),
        "GetTextFaceW" => GetTextFaceW(emu),
        "GetWorldTransform" => GetWorldTransform(emu),
        "IntersectClipRect" => IntersectClipRect(emu),
        "PatBlt" => PatBlt(emu),
        "Pie" => GdiPie(emu),
        "PolyBezier" => GdiPolyBezier(emu),
        "Polygon" => GdiPolygon(emu),
        "Polyline" => GdiPolyline(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "RemoveFontResourceExA" => RemoveFontResourceExA(emu),
        "RemoveFontResourceExW" => RemoveFontResourceExW(emu),
        "SelectClipRgn" => SelectClipRgn(emu),
        "SetDCBrushColor" => SetDCBrushColor(emu),
        "SetDCPenColor" => SetDCPenColor(emu),
        "SetDIBits" => SetDIBits(emu),
        "SetLayout" => SetLayout(emu),
        "SetMapMode" => SetMapMode(emu),
        "SetPolyFillMode" => SetPolyFillMode(emu),
        "SetROP2" => SetROP2(emu),
        "SetTextAlign" => SetTextAlign(emu),
        "SetViewportOrgEx" => SetViewportOrgEx(emu),
        "SetWindowOrgEx" => SetWindowOrgEx(emu),
        "SetWorldTransform" => SetWorldTransform(emu),
        "StartDoc" | "StartDocA" | "StartDocW" => StartDoc(emu),
        "StartPage" => StartPage(emu),
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

// === DC operations ===

pub fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    emu.regs_mut().rax = helper::handler_create("gdi32://CreateCompatibleDC");
}

pub fn CreateDCA(emu: &mut emu::Emu) {
    let driver_ptr = emu.regs().rcx;
    let device_ptr = emu.regs().rdx;
    let driver_name = if driver_ptr != 0 {
        emu.maps.read_string(driver_ptr)
    } else {
        String::new()
    };
    let device_name = if device_ptr != 0 {
        emu.maps.read_string(device_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateDCA driver:'{}' device:'{}'",
        driver_name,
        device_name
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    let driver_ptr = emu.regs().rcx;
    let device_ptr = emu.regs().rdx;
    let driver_name = if driver_ptr != 0 {
        emu.maps.read_wide_string(driver_ptr)
    } else {
        String::new()
    };
    let device_name = if device_ptr != 0 {
        emu.maps.read_wide_string(device_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateDCW driver:'{}' device:'{}'",
        driver_name,
        device_name
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}

pub fn DeleteDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "gdi32!GetDC hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}

pub fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = emu.regs().rdx;
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
}

pub fn SaveDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!SaveDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}

pub fn RestoreDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let saved = emu.regs().rdx as i32;
    log_red!(emu, "gdi32!RestoreDC hdc:0x{:x} saved:{}", hdc, saved);
    if helper::handler_exist(hdc) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}

// === Bitmap ===

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let width = emu.regs().rdx;
    let height = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!CreateCompatibleBitmap hdc:0x{:x} w:{} h:{}",
        hdc,
        width,
        height
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://CreateCompatibleBitmap");
}

pub fn CreateDIBSection(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let pbmi = emu.regs().rdx;
    let usage = emu.regs().r8;
    let ppv_bits = emu.regs().r9;
    log_red!(
        emu,
        "gdi32!CreateDIBSection hdc:0x{:x} bmi:0x{:x} usage:{} bits_out:0x{:x}",
        hdc,
        pbmi,
        usage,
        ppv_bits
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://bitmap");
}

pub fn BitBlt(emu: &mut emu::Emu) {
    let hdc_dest = emu.regs().rcx;
    let x_dest = emu.regs().rdx as i32;
    let y_dest = emu.regs().r8 as i32;
    let width = emu.regs().r9 as i32;
    let height = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let hdc_src = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let x_src = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let y_src = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let rop = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32;
    log_red!(
        emu,
        "gdi32!BitBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        width,
        height,
        hdc_src,
        x_src,
        y_src,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn StretchBlt(emu: &mut emu::Emu) {
    let hdc_dest = emu.regs().rcx;
    let x_dest = emu.regs().rdx as i32;
    let y_dest = emu.regs().r8 as i32;
    let w_dest = emu.regs().r9 as i32;
    let h_dest = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let hdc_src = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let x_src = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let y_src = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let w_src = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    let h_src = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as i32;
    let rop = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0) as u32;
    log_red!(
        emu,
        "gdi32!StretchBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} {}x{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        w_dest,
        h_dest,
        hdc_src,
        x_src,
        y_src,
        w_src,
        h_src,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PatBlt(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let w = emu.regs().r9 as i32;
    let h = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let rop = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    log_red!(
        emu,
        "gdi32!PatBlt hdc:0x{:x} x:{} y:{} w:{} h:{} rop:0x{:x}",
        hdc,
        x,
        y,
        w,
        h,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetDIBits(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hbmp = emu.regs().rdx;
    let start = emu.regs().r8;
    let lines = emu.regs().r9;
    log_red!(
        emu,
        "gdi32!SetDIBits hdc:0x{:x} hbmp:0x{:x} start:{} lines:{}",
        hdc,
        hbmp,
        start,
        lines
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = lines;
}

pub fn GetDIBits(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hbmp = emu.regs().rdx;
    let start = emu.regs().r8;
    let lines = emu.regs().r9;
    let lpvbits = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let lpbmi = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if lpbmi != 0 {
        emu.maps.write_dword(lpbmi, 40); // biSize = BITMAPINFOHEADER
        emu.maps.write_dword(lpbmi + 4, 1920); // biWidth
        emu.maps.write_dword(lpbmi + 8, 1080u32 as i32 as u32); // biHeight
        emu.maps.write_word(lpbmi + 12, 1); // biPlanes
        emu.maps.write_word(lpbmi + 14, 32); // biBitCount
    }
    log_red!(
        emu,
        "gdi32!GetDIBits hdc:0x{:x} hbmp:0x{:x} start:{} lines:{} bits:0x{:x} bmi:0x{:x}",
        hdc,
        hbmp,
        start,
        lines,
        lpvbits,
        lpbmi
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = lines;
}

pub fn GetDIBColorTable(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let start = emu.regs().rdx;
    let entries = emu.regs().r8;
    let colors = emu.regs().r9;
    if colors != 0 && entries > 0 {
        let n = entries.min(256) as usize;
        for i in 0..n {
            let offset = colors + (i as u64) * 4;
            let gray = (i * 255 / n.max(1)) as u32;
            let rgbq = gray | (gray << 8) | (gray << 16);
            emu.maps.write_dword(offset, rgbq);
        }
    }
    log_red!(
        emu,
        "gdi32!GetDIBColorTable hdc:0x{:x} start:{} entries:{} colors:0x{:x}",
        hdc,
        start,
        entries,
        colors
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = entries.min(256);
}

pub fn SetBkMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    let mode_name = match mode {
        1 => "TRANSPARENT",
        2 => "OPAQUE",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!SetBkMode hdc:0x{:x} mode:{} ({})",
        hdc,
        mode,
        mode_name
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetBkColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetBkColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00ff_ffff;
}

pub fn GetBkColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetBkColor hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00ff_ffff;
}

pub fn SetStretchBltMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    log_red!(emu, "gdi32!SetStretchBltMode hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Object ===

pub fn SelectObject(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let obj = emu.regs().rdx;
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    if !helper::handler_exist(hdc) {
        log_red!(emu, "gdi32!SelectObject invalid hdc");
        emu.regs_mut().rax = 0;
        return;
    }
    if !helper::handler_exist(obj) {
        log_red!(emu, "gdi32!SelectObject invalid obj");
        emu.regs_mut().rax = 0;
        return;
    }
    // Return previous object (simulate by returning the obj being selected)
    emu.regs_mut().rax = obj;
}

pub fn DeleteObject(emu: &mut emu::Emu) {
    let obj = emu.regs().rcx;
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::handler_close(obj);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetObjectA(emu: &mut emu::Emu) {
    let h = emu.regs().rcx;
    let cb = emu.regs().rdx as u32;
    let pv = emu.regs().r8;
    log_red!(emu, "gdi32!GetObjectA h:0x{:x} cb:{} pv:0x{:x}", h, cb, pv);
    if pv != 0 && cb > 0 {
        // Zero-fill the buffer first
        let fill = (cb as u64).min(64);
        for i in 0..fill {
            emu.maps.write_byte(pv + i, 0);
        }
        // Fill as BITMAP structure if buffer is large enough
        if cb >= 24 {
            emu.maps.write_dword(pv, 0); // bmType
            emu.maps.write_dword(pv + 4, 64); // bmWidth
            emu.maps.write_dword(pv + 8, 64); // bmHeight
            emu.maps.write_dword(pv + 12, 256); // bmWidthBytes
            emu.maps.write_word(pv + 16, 1); // bmPlanes
            emu.maps.write_word(pv + 18, 32); // bmBitsPixel
        }
        emu.regs_mut().rax = fill;
    } else {
        // When pv is NULL, return the required buffer size
        emu.regs_mut().rax = 24; // BITMAP struct size
    }
}

pub fn GetObjectW(emu: &mut emu::Emu) {
    let h = emu.regs().rcx;
    let cb = emu.regs().rdx as u32;
    let pv = emu.regs().r8;
    log_red!(emu, "gdi32!GetObjectW h:0x{:x} cb:{} pv:0x{:x}", h, cb, pv);
    if pv != 0 && cb > 0 {
        let fill = (cb as u64).min(64);
        for i in 0..fill {
            emu.maps.write_byte(pv + i, 0);
        }
        if cb >= 24 {
            emu.maps.write_dword(pv, 0); // bmType
            emu.maps.write_dword(pv + 4, 64); // bmWidth
            emu.maps.write_dword(pv + 8, 64); // bmHeight
            emu.maps.write_dword(pv + 12, 256); // bmWidthBytes
            emu.maps.write_word(pv + 16, 1); // bmPlanes
            emu.maps.write_word(pv + 18, 32); // bmBitsPixel
        }
        emu.regs_mut().rax = fill;
    } else {
        emu.regs_mut().rax = 24;
    }
}

pub fn GetObjectType(emu: &mut emu::Emu) {
    let obj = emu.regs().rcx;
    let result: u64 = if helper::handler_exist(obj) { 6 } else { 0 }; // OBJ_BITMAP=6 as generic
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x} = {}", obj, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}

pub fn GetStockObject(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let name = match index {
        0 => "WHITE_BRUSH",
        1 => "LTGRAY_BRUSH",
        2 => "GRAY_BRUSH",
        3 => "DKGRAY_BRUSH",
        4 => "BLACK_BRUSH",
        5 => "NULL_BRUSH",
        6 => "WHITE_PEN",
        7 => "BLACK_PEN",
        8 => "NULL_PEN",
        10 => "OEM_FIXED_FONT",
        11 => "ANSI_FIXED_FONT",
        12 => "ANSI_VAR_FONT",
        13 => "SYSTEM_FONT",
        14 => "DEVICE_DEFAULT_FONT",
        15 => "DEFAULT_PALETTE",
        16 => "SYSTEM_FIXED_FONT",
        17 => "DEFAULT_GUI_FONT",
        18 => "DC_BRUSH",
        19 => "DC_PEN",
        _ => "UNKNOWN",
    };
    log_red!(emu, "gdi32!GetStockObject index:{} ({})", index, name);
    emu.regs_mut().rax = helper::handler_create("gdi32://stock_object");
}

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let obj_type = emu.regs().rdx;
    let tag = match obj_type {
        1 => "gdi32://pen",
        2 => "gdi32://brush",
        6 => "gdi32://font",
        7 => "gdi32://bitmap",
        _ => "",
    };
    let result: u64 = if tag.is_empty() {
        0
    } else {
        helper::handler_create(tag)
    };
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{} = 0x{:x}",
        hdc,
        obj_type,
        result
    );
    emu.regs_mut().rax = result;
}

pub fn GetDeviceCaps(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let index = emu.regs().rdx as i32;
    // Windows GetDeviceCaps index constants (wingdi.h)
    let result: u64 = match index {
        0 => 1,            // DRIVERVERSION
        2 => 0,            // TECHNOLOGY (DT_RASDISPLAY)
        4 => 508,          // HORZSIZE (mm, ~20" for 1920px at 96dpi)
        6 => 285,          // VERTSIZE (mm, ~11.25" for 1080px at 96dpi)
        8 => 1920,         // HORZRES (pixels)
        10 => 1080,        // VERTRES (pixels)
        12 => 32,          // BITSPIXEL (bits per pixel)
        14 => 32,          // BITSPIXEL alias (some callers use this index)
        16 => 0xFFFF_FFFF, // NUMBRUSHES (-1 as DWORD = device-supported)
        18 => 0xFFFF_FFFF, // NUMPENS (-1 as DWORD = device-supported)
        20 => 0,           // NUMMARKERS
        22 => 0,           // NUMFONTS
        24 => 0,           // NUMCOLORS (0 for >8bpp)
        26 => 0,           // PDEVICESIZE
        28 => 0x1ff,       // CURVECAPS (all curve capabilities)
        30 => 0x1ff,       // LINECAPS (all line capabilities)
        32 => 0x1ff,       // POLYGONALCAPS (all polygon capabilities)
        34 => 0x7fff,      // TEXTCAPS (all text capabilities)
        36 => 1,           // CLIPCAPS (CP_RECTANGLE)
        38 => 0x7e99,      // RASTERCAPS
        40 => 1,           // ASPECTX (relative width of pixel)
        42 => 1,           // ASPECTY (relative height of pixel)
        44 => 1,           // ASPECTXY (diagonal aspect ratio)
        88 => 96,          // LOGPIXELSX (logical pixels per inch X)
        90 => 96,          // LOGPIXELSY (logical pixels per inch Y)
        104 => 256,        // SIZEPALETTE
        106 => 20,         // NUMRESERVED
        108 => 18,         // COLORRES (bits of color resolution)
        110 => 0,          // PHYSICALWIDTH (printer only)
        111 => 0,          // PHYSICALHEIGHT (printer only)
        112 => 0,          // PHYSICALOFFSETX (printer only)
        113 => 0,          // PHYSICALOFFSETY (printer only)
        114 => 0,          // SCALINGFACTORX
        115 => 0,          // SCALINGFACTORY
        116 => 60,         // VREFRESH (Hz)
        117 => 1080,       // DESKTOPVERTRES
        118 => 1920,       // DESKTOPHORZRES
        119 => 0,          // BLTALIGNMENT
        120 => 0x1ff,      // SHADEBLENDCAPS
        121 => 0x7f,       // COLORMGMTCAPS
        _ => 0,
    };
    log_red!(
        emu,
        "gdi32!GetDeviceCaps hdc:0x{:x} index:{} = {}",
        hdc,
        index,
        result
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}

// === Font ===

pub fn CreateFontA(emu: &mut emu::Emu) {
    let height = emu.regs().rcx as i32;
    let width = emu.regs().rdx as i32;
    let escapement = emu.regs().r8 as i32;
    let orientation = emu.regs().r9 as i32;
    let weight = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let italic = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let underline = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let strikeout = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as u32;
    let charset = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32;
    let out_precision = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as u32;
    let clip_precision = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0) as u32;
    let quality = emu.maps.read_qword(emu.regs().rsp + 0x60).unwrap_or(0) as u32;
    let pitch_and_family = emu.maps.read_qword(emu.regs().rsp + 0x68).unwrap_or(0) as u32;
    let face_ptr = emu.maps.read_qword(emu.regs().rsp + 0x70).unwrap_or(0);
    let face_name = if face_ptr != 0 {
        emu.maps.read_string(face_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateFontA h:{} w:{} weight:{} italic:{} charset:{} quality:{} face:'{}'",
        height,
        width,
        weight,
        italic,
        charset,
        quality,
        face_name
    );
    let _ = (
        escapement,
        orientation,
        underline,
        strikeout,
        out_precision,
        clip_precision,
        pitch_and_family,
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    let height = emu.regs().rcx as i32;
    let width = emu.regs().rdx as i32;
    let escapement = emu.regs().r8 as i32;
    let orientation = emu.regs().r9 as i32;
    let weight = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let italic = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let underline = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let strikeout = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as u32;
    let charset = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32;
    let out_precision = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as u32;
    let clip_precision = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0) as u32;
    let quality = emu.maps.read_qword(emu.regs().rsp + 0x60).unwrap_or(0) as u32;
    let pitch_and_family = emu.maps.read_qword(emu.regs().rsp + 0x68).unwrap_or(0) as u32;
    let face_ptr = emu.maps.read_qword(emu.regs().rsp + 0x70).unwrap_or(0);
    let face_name = if face_ptr != 0 {
        emu.maps.read_wide_string(face_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateFontW h:{} w:{} weight:{} italic:{} charset:{} quality:{} face:'{}'",
        height,
        width,
        weight,
        italic,
        charset,
        quality,
        face_name
    );
    let _ = (
        escapement,
        orientation,
        underline,
        strikeout,
        out_precision,
        clip_precision,
        pitch_and_family,
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    let lf = emu.regs().rcx;
    let height = if lf != 0 {
        emu.maps.read_dword(lf).unwrap_or(0) as i32
    } else {
        0
    };
    log_red!(
        emu,
        "gdi32!CreateFontIndirectA lf:0x{:x} height:{}",
        lf,
        height
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    let lf = emu.regs().rcx;
    let height = if lf != 0 {
        emu.maps.read_dword(lf).unwrap_or(0) as i32
    } else {
        0
    };
    log_red!(
        emu,
        "gdi32!CreateFontIndirectW lf:0x{:x} height:{}",
        lf,
        height
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn GetTextMetricsA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
        emu.maps.write_dword(tm + 28, 400); // tmWeight (FW_NORMAL)
        emu.maps.write_dword(tm + 32, 0); // tmOverhang
        emu.maps.write_dword(tm + 36, 96); // tmDigitizedAspectX
        emu.maps.write_dword(tm + 40, 96); // tmDigitizedAspectY
        emu.maps.write_byte(tm + 44, 0x20); // tmFirstChar
        emu.maps.write_byte(tm + 45, 0xff); // tmLastChar
        emu.maps.write_byte(tm + 46, 0x3f); // tmDefaultChar ('?')
        emu.maps.write_byte(tm + 47, 0x20); // tmBreakChar (' ')
        emu.maps.write_byte(tm + 48, 0); // tmItalic
        emu.maps.write_byte(tm + 49, 0); // tmUnderlined
        emu.maps.write_byte(tm + 50, 0); // tmStruckOut
        emu.maps.write_byte(tm + 51, 0); // tmPitchAndFamily
        emu.maps.write_byte(tm + 52, 0); // tmCharSet (ANSI_CHARSET)
    }
    log_red!(emu, "gdi32!GetTextMetricsA hdc:0x{:x} tm:0x{:x}", hdc, tm);
    emu.regs_mut().rax = 1;
}

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
        emu.maps.write_dword(tm + 28, 400); // tmWeight (FW_NORMAL)
        emu.maps.write_dword(tm + 32, 0); // tmOverhang
        emu.maps.write_dword(tm + 36, 96); // tmDigitizedAspectX
        emu.maps.write_dword(tm + 40, 96); // tmDigitizedAspectY
        emu.maps.write_word(tm + 44, 0x0020); // tmFirstChar (WCHAR)
        emu.maps.write_word(tm + 46, 0xffff); // tmLastChar
        emu.maps.write_word(tm + 48, 0x003f); // tmDefaultChar ('?')
        emu.maps.write_word(tm + 50, 0x0020); // tmBreakChar (' ')
        emu.maps.write_byte(tm + 52, 0); // tmItalic
        emu.maps.write_byte(tm + 53, 0); // tmUnderlined
        emu.maps.write_byte(tm + 54, 0); // tmStruckOut
        emu.maps.write_byte(tm + 55, 0); // tmPitchAndFamily
        emu.maps.write_byte(tm + 56, 0); // tmCharSet (ANSI_CHARSET)
    }
    log_red!(emu, "gdi32!GetTextMetricsW hdc:0x{:x} tm:0x{:x}", hdc, tm);
    emu.regs_mut().rax = 1;
}

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    let text = if text_ptr != 0 && len > 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32); // cx
        emu.maps.write_dword(size + 4, 16); // cy
    }
    log_red!(
        emu,
        "gdi32!GetTextExtentPoint32A hdc:0x{:x} text:'{}' len:{} => {}x16",
        hdc,
        text,
        len,
        len * 7
    );
    emu.regs_mut().rax = 1;
}

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    let text = if text_ptr != 0 && len > 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32);
        emu.maps.write_dword(size + 4, 16);
    }
    log_red!(
        emu,
        "gdi32!GetTextExtentPoint32W hdc:0x{:x} text:'{}' len:{} => {}x16",
        hdc,
        text,
        len,
        len * 7
    );
    emu.regs_mut().rax = 1;
}

pub fn GetTextFaceA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let count = emu.regs().rdx;
    let buf = emu.regs().r8;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_string(buf, face);
    }
    log_red!(
        emu,
        "gdi32!GetTextFaceA hdc:0x{:x} count:{} => '{}'",
        hdc,
        count,
        face
    );
    emu.regs_mut().rax = face.len() as u64;
}

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let count = emu.regs().rdx;
    let buf = emu.regs().r8;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_wide_string(buf, face);
    }
    log_red!(
        emu,
        "gdi32!GetTextFaceW hdc:0x{:x} count:{} => '{}'",
        hdc,
        count,
        face
    );
    emu.regs_mut().rax = face.len() as u64;
}

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    let name = emu.maps.read_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!AddFontResourceExA '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!AddFontResourceExW '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    let name = emu.maps.read_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExA '{}' flags:0x{:x}",
        name,
        fl
    );
    emu.regs_mut().rax = 1;
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExW '{}' flags:0x{:x}",
        name,
        fl
    );
    emu.regs_mut().rax = 1;
}

pub fn GetCharWidth32A(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let buf = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + i * 4, 8);
        }
    }
    log_red!(
        emu,
        "gdi32!GetCharWidth32A hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    emu.regs_mut().rax = 1;
}

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let buf = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + i * 4, 8);
        }
    }
    log_red!(
        emu,
        "gdi32!GetCharWidth32W hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    emu.regs_mut().rax = 1;
}

pub fn GetCharABCWidthsA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let abc = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if abc != 0 {
        for i in 0..count {
            let off = abc + i * 12;
            emu.maps.write_dword(off, 1); // abcA
            emu.maps.write_dword(off + 4, 6); // abcB
            emu.maps.write_dword(off + 8, 1); // abcC
        }
    }
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsA hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    emu.regs_mut().rax = 1;
}

pub fn GetCharABCWidthsW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let abc = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if abc != 0 {
        for i in 0..count {
            let off = abc + i * 12;
            emu.maps.write_dword(off, 1); // abcA
            emu.maps.write_dword(off + 4, 6); // abcB
            emu.maps.write_dword(off + 8, 1); // abcC
        }
    }
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsW hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    emu.regs_mut().rax = 1;
}

pub fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let logfont = emu.regs().rdx;
    let proc_ptr = emu.regs().r8;
    let lparam = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!EnumFontFamiliesExA hdc:0x{:x} lf:0x{:x} proc:0x{:x} lparam:0x{:x}",
        hdc,
        logfont,
        proc_ptr,
        lparam
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let logfont = emu.regs().rdx;
    let proc_ptr = emu.regs().r8;
    let lparam = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!EnumFontFamiliesExW hdc:0x{:x} lf:0x{:x} proc:0x{:x} lparam:0x{:x}",
        hdc,
        logfont,
        proc_ptr,
        lparam
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Text ===

pub fn SetTextColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetTextColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn GetTextColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetTextColor hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn SetTextAlign(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let align = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetTextAlign hdc:0x{:x} align:0x{:x}",
        hdc,
        align
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous alignment (TA_LEFT | TA_TOP)
}

pub fn GetTextAlign(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetTextAlign hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // TA_LEFT | TA_TOP
}

pub fn TextOutA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let text_ptr = emu.regs().r9;
    let count = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let text = if text_ptr != 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!TextOutA hdc:0x{:x} x:{} y:{} count:{} text:'{}'",
        hdc,
        x,
        y,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

pub fn TextOutW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let text_ptr = emu.regs().r9;
    let count = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let text = if text_ptr != 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!TextOutW hdc:0x{:x} x:{} y:{} count:{} text:'{}'",
        hdc,
        x,
        y,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

pub fn ExtTextOutA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let options = emu.regs().r9 as u32;
    let _lprect = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let count = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let text = if text_ptr != 0 && count > 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!ExtTextOutA hdc:0x{:x} x:{} y:{} options:0x{:x} count:{} text:'{}'",
        hdc,
        x,
        y,
        options,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let options = emu.regs().r9 as u32;
    let _lprect = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let count = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let text = if text_ptr != 0 && count > 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!ExtTextOutW hdc:0x{:x} x:{} y:{} options:0x{:x} count:{} text:'{}'",
        hdc,
        x,
        y,
        options,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

// === Drawing ===

pub fn GdiRectangle(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Rectangle hdc:0x{:x} left:{} top:{} right:{} bottom:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiEllipse(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Ellipse hdc:0x{:x} left:{} top:{} right:{} bottom:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn MoveToEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let lp_point = emu.regs().r9;
    // If lpPoint is non-null, write the previous position (0,0)
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!MoveToEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    emu.regs_mut().rax = 1;
}

pub fn LineTo(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    log_red!(emu, "gdi32!LineTo hdc:0x{:x} x:{} y:{}", hdc, x, y);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetPixel(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let color = emu.regs().r9 as u32;
    log_red!(
        emu,
        "gdi32!SetPixel hdc:0x{:x} x:{} y:{} color:0x{:06x}",
        hdc,
        x,
        y,
        color
    );
    // Return the RGB color that was set (or CLR_INVALID on failure)
    emu.set_last_error(0);
    emu.regs_mut().rax = color as u64;
}

pub fn GetPixelGdi(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    log_red!(emu, "gdi32!GetPixel hdc:0x{:x} x:{} y:{}", hdc, x, y);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // black pixel
}

pub fn GdiFillRect(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let rect = emu.regs().rdx;
    let brush = emu.regs().r8;
    let (left, top, right, bottom) = if rect != 0 {
        (
            emu.maps.read_dword(rect).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 8).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 12).unwrap_or(0) as i32,
        )
    } else {
        (0, 0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!FillRect hdc:0x{:x} rect:({},{},{},{}) brush:0x{:x}",
        hdc,
        left,
        top,
        right,
        bottom,
        brush
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiPolygon(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let points = emu.regs().rdx;
    let count = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!Polygon hdc:0x{:x} pts:0x{:x} count:{}",
        hdc,
        points,
        count
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiPolyline(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let points = emu.regs().rdx;
    let count = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!Polyline hdc:0x{:x} pts:0x{:x} count:{}",
        hdc,
        points,
        count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiPolyBezier(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let points = emu.regs().rdx;
    let count = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!PolyBezier hdc:0x{:x} pts:0x{:x} count:{}",
        hdc,
        points,
        count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiArc(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let x_start = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let y_start = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let x_end = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let y_end = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Arc hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiPie(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let x_start = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let y_start = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let x_end = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let y_end = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Pie hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiChord(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let x_start = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let y_start = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let x_end = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let y_end = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Chord hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Pen/Brush ===

pub fn CreatePen(emu: &mut emu::Emu) {
    let style = emu.regs().rcx as u32;
    let width = emu.regs().rdx as i32;
    let color = emu.regs().r8 as u32;
    let style_name = match style {
        0 => "PS_SOLID",
        1 => "PS_DASH",
        2 => "PS_DOT",
        3 => "PS_DASHDOT",
        4 => "PS_DASHDOTDOT",
        5 => "PS_NULL",
        6 => "PS_INSIDEFRAME",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CreatePen style:{} ({}) width:{} color:0x{:06x}",
        style,
        style_name,
        width,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://pen");
}

pub fn CreatePenIndirect(emu: &mut emu::Emu) {
    let lpen = emu.regs().rcx;
    let (style, width, color) = if lpen != 0 {
        (
            emu.maps.read_dword(lpen).unwrap_or(0),
            emu.maps.read_dword(lpen + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(lpen + 12).unwrap_or(0),
        )
    } else {
        (0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreatePenIndirect lpen:0x{:x} style:{} width:{} color:0x{:06x}",
        lpen,
        style,
        width,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://pen");
}

pub fn CreateSolidBrush(emu: &mut emu::Emu) {
    let color = emu.regs().rcx as u32;
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:06x}", color);
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}

pub fn CreateBrushIndirect(emu: &mut emu::Emu) {
    let lbrush = emu.regs().rcx;
    let (style, color) = if lbrush != 0 {
        (
            emu.maps.read_dword(lbrush).unwrap_or(0),
            emu.maps.read_dword(lbrush + 4).unwrap_or(0),
        )
    } else {
        (0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreateBrushIndirect lbrush:0x{:x} style:{} color:0x{:06x}",
        lbrush,
        style,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}

pub fn CreatePatternBrush(emu: &mut emu::Emu) {
    let hbmp = emu.regs().rcx;
    log_red!(emu, "gdi32!CreatePatternBrush hbmp:0x{:x}", hbmp);
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}

pub fn CreateHatchBrush(emu: &mut emu::Emu) {
    let style = emu.regs().rcx as u32;
    let color = emu.regs().rdx as u32;
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}

// === Region ===

pub fn CreateRectRgn(emu: &mut emu::Emu) {
    let x1 = emu.regs().rcx as i32;
    let y1 = emu.regs().rdx as i32;
    let x2 = emu.regs().r8 as i32;
    let y2 = emu.regs().r9 as i32;
    log_red!(
        emu,
        "gdi32!CreateRectRgn x1:{} y1:{} x2:{} y2:{}",
        x1,
        y1,
        x2,
        y2
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://region");
}

pub fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    let (left, top, right, bottom) = if rect != 0 {
        (
            emu.maps.read_dword(rect).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 8).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 12).unwrap_or(0) as i32,
        )
    } else {
        (0, 0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreateRectRgnIndirect rect:({},{},{},{})",
        left,
        top,
        right,
        bottom
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://region");
}

pub fn GetClipBox(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let rect = emu.regs().rdx;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "gdi32!GetClipBox hdc:0x{:x} rect:0x{:x}", hdc, rect);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}

pub fn CombineRgn(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let src1 = emu.regs().rdx;
    let src2 = emu.regs().r8;
    let mode = emu.regs().r9 as u32;
    let mode_name = match mode {
        1 => "RGN_AND",
        2 => "RGN_OR",
        3 => "RGN_XOR",
        4 => "RGN_DIFF",
        5 => "RGN_COPY",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CombineRgn dest:0x{:x} src1:0x{:x} src2:0x{:x} mode:{} ({})",
        dest,
        src1,
        src2,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}

pub fn SelectClipRgn(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hrgn = emu.regs().rdx;
    log_red!(emu, "gdi32!SelectClipRgn hdc:0x{:x} hrgn:0x{:x}", hdc, hrgn);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}

pub fn IntersectClipRect(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!IntersectClipRect hdc:0x{:x} l:{} t:{} r:{} b:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}

pub fn ExcludeClipRect(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!ExcludeClipRect hdc:0x{:x} l:{} t:{} r:{} b:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}

// === Map/Viewport ===

pub fn SetMapMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    let mode_name = match mode {
        1 => "MM_TEXT",
        2 => "MM_LOMETRIC",
        3 => "MM_HIMETRIC",
        4 => "MM_LOENGLISH",
        5 => "MM_HIENGLISH",
        6 => "MM_TWIPS",
        7 => "MM_ISOTROPIC",
        8 => "MM_ANISOTROPIC",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!SetMapMode hdc:0x{:x} mode:{} ({})",
        hdc,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode: MM_TEXT
}

pub fn GetMapMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetMapMode hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // MM_TEXT
}

pub fn SetWindowOrgEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let lp_point = emu.regs().r9;
    // Write previous origin to lpPoint if non-null
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!SetWindowOrgEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    emu.regs_mut().rax = 1;
}

pub fn SetViewportOrgEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let lp_point = emu.regs().r9;
    // Write previous origin to lpPoint if non-null
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!SetViewportOrgEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    emu.regs_mut().rax = 1;
}

// === ROP/mode ===

pub fn SetROP2(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    log_red!(emu, "gdi32!SetROP2 hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 13; // previous: R2_COPYPEN
}

pub fn GetROP2(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetROP2 hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 13; // R2_COPYPEN
}

pub fn GetStretchBltModeGdi(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetStretchBltMode hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // BLACKONWHITE
}

pub fn SetPolyFillMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    log_red!(emu, "gdi32!SetPolyFillMode hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous: ALTERNATE
}

pub fn GetPolyFillMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetPolyFillMode hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // ALTERNATE
}

// === Transform ===

pub fn SetWorldTransform(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let xform = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetWorldTransform(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let xform = emu.regs().rdx;
    if xform != 0 {
        // Identity matrix: eM11=1.0f, eM12=0, eM21=0, eM22=1.0f, eDx=0, eDy=0
        emu.maps.write_dword(xform, 0x3f80_0000); // 1.0f eM11
        emu.maps.write_dword(xform + 4, 0); // eM12
        emu.maps.write_dword(xform + 8, 0); // eM21
        emu.maps.write_dword(xform + 12, 0x3f80_0000); // 1.0f eM22
        emu.maps.write_dword(xform + 16, 0); // eDx
        emu.maps.write_dword(xform + 20, 0); // eDy
    }
    log_red!(
        emu,
        "gdi32!GetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    emu.regs_mut().rax = 1;
}

// === Printer ===

pub fn StartDoc(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let docinfo = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!StartDoc hdc:0x{:x} docinfo:0x{:x}",
        hdc,
        docinfo
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // job id
}

pub fn EndDoc(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!EndDoc hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn StartPage(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!StartPage hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EndPage(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!EndPage hdc:0x{:x}", hdc);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Palette ===

pub fn GetSystemPaletteEntries(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let start = emu.regs().rdx as u32;
    let entries = emu.regs().r8 as u32;
    let pe = emu.regs().r9;
    if pe != 0 && entries > 0 {
        for i in 0..entries.min(256) {
            let offset = pe + (i as u64) * 4;
            let gray = ((start + i) * 255 / 256) as u8;
            let entry = (gray as u32) | ((gray as u32) << 8) | ((gray as u32) << 16);
            emu.maps.write_dword(offset, entry);
        }
    }
    log_red!(
        emu,
        "gdi32!GetSystemPaletteEntries hdc:0x{:x} start:{} entries:{} pe:0x{:x}",
        hdc,
        start,
        entries,
        pe
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = entries.min(256) as u64;
}

pub fn GetNearestColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!GetNearestColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = color & 0x00ff_ffff; // nearest == exact in 24-bit
}

// === DC Colors ===

pub fn SetDCBrushColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetDCBrushColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00ff_ffff; // previous color (white)
}

pub fn SetDCPenColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetDCPenColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous color (black)
}

// === Layout ===

pub fn GetLayout(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetLayout hdc:0x{:x}", hdc);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // LAYOUT_LTR (default left-to-right)
}

pub fn SetLayout(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let layout = emu.regs().rdx;
    log_red!(emu, "gdi32!SetLayout hdc:0x{:x} layout:0x{:x}", hdc, layout);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous layout
}
