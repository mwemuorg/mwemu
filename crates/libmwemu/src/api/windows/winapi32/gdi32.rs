use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        // DC operations
        "CreateDCA" => CreateDCA(emu),
        "CreateDCW" => CreateDCW(emu),
        "CreateCompatibleDC" => CreateCompatibleDC(emu),
        "DeleteDC" => DeleteDC(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "SaveDC" => SaveDC(emu),
        "RestoreDC" => RestoreDC(emu),

        // Bitmap
        "CreateCompatibleBitmap" => CreateCompatibleBitmap(emu),
        "CreateDIBSection" => CreateDIBSection(emu),
        "BitBlt" => BitBlt(emu),
        "StretchBlt" => StretchBlt(emu),
        "PatBlt" => PatBlt(emu),
        "SetDIBits" => SetDIBits(emu),
        "GetDIBits" => GetDIBits(emu),
        "GetDIBColorTable" => GetDIBColorTable(emu),
        "SetBkMode" => SetBkMode(emu),
        "SetBkColor" => SetBkColor(emu),

        // Object
        "SelectObject" => SelectObject(emu),
        "DeleteObject" => DeleteObject(emu),
        "GetObjectA" => GetObjectA(emu),
        "GetObjectW" => GetObjectW(emu),
        "GetObjectType" => GetObjectType(emu),
        "GetStockObject" => GetStockObject(emu),
        "GetCurrentObject" => GetCurrentObject(emu),
        "GetDeviceCaps" => GetDeviceCaps(emu),

        // Font
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        "CreateFontIndirectW" => CreateFontIndirectW(emu),
        "CreateFontA" => CreateFontA(emu),
        "CreateFontW" => CreateFontW(emu),
        "GetTextMetricsA" => GetTextMetricsA(emu),
        "GetTextMetricsW" => GetTextMetricsW(emu),
        "GetTextExtentPoint32A" => GetTextExtentPoint32A(emu),
        "GetTextExtentPoint32W" => GetTextExtentPoint32W(emu),
        "GetTextFaceA" => GetTextFaceA(emu),
        "GetTextFaceW" => GetTextFaceW(emu),
        "AddFontResourceExA" => AddFontResourceExA(emu),
        "AddFontResourceExW" => AddFontResourceExW(emu),
        "RemoveFontResourceExA" => RemoveFontResourceExA(emu),
        "RemoveFontResourceExW" => RemoveFontResourceExW(emu),
        "GetCharWidth32A" => GetCharWidth32A(emu),
        "GetCharWidth32W" => GetCharWidth32W(emu),
        "GetCharABCWidthsA" => GetCharABCWidthsA(emu),
        "GetCharABCWidthsW" => GetCharABCWidthsW(emu),
        "EnumFontFamiliesExA" => EnumFontFamiliesExA(emu),
        "EnumFontFamiliesExW" => EnumFontFamiliesExW(emu),

        // Text
        "TextOutA" => TextOutA(emu),
        "TextOutW" => TextOutW(emu),
        "ExtTextOutA" => ExtTextOutA(emu),
        "ExtTextOutW" => ExtTextOutW(emu),
        "SetTextColor" => SetTextColor(emu),
        "GetTextColor" => GetTextColor(emu),
        "SetTextAlign" => SetTextAlign(emu),
        "GetTextAlign" => GetTextAlign(emu),

        // Drawing
        "Rectangle" => GdiRectangle(emu),
        "Ellipse" => GdiEllipse(emu),
        "MoveToEx" => MoveToEx(emu),
        "LineTo" => LineTo(emu),
        "SetPixel" => SetPixel(emu),
        "GetPixel" => GetPixelGdi(emu),
        "FillRect" => FillRect(emu),
        "Polygon" => GdiPolygon(emu),
        "Polyline" => GdiPolyline(emu),
        "PolyBezier" => GdiPolyBezier(emu),
        "Arc" => GdiArc(emu),
        "Pie" => GdiPie(emu),
        "Chord" => GdiChord(emu),

        // Region
        "CreateRectRgn" => CreateRectRgn(emu),
        "CreateRectRgnIndirect" => CreateRectRgnIndirect(emu),
        "GetClipBox" => GetClipBox(emu),
        "CombineRgn" => CombineRgn(emu),
        "SelectClipRgn" => SelectClipRgn(emu),
        "IntersectClipRect" => IntersectClipRect(emu),
        "ExcludeClipRect" => ExcludeClipRect(emu),

        // Pen/Brush
        "CreatePen" => CreatePen(emu),
        "CreatePenIndirect" => CreatePenIndirect(emu),
        "CreateSolidBrush" => CreateSolidBrush(emu),
        "CreateBrushIndirect" => CreateBrushIndirect(emu),
        "CreatePatternBrush" => CreatePatternBrush(emu),
        "CreateHatchBrush" => CreateHatchBrush(emu),

        // Map/viewport
        "SetMapMode" => SetMapMode(emu),
        "GetMapMode" => GetMapMode(emu),
        "SetWindowOrgEx" => SetWindowOrgEx(emu),
        "SetViewportOrgEx" => SetViewportOrgEx(emu),

        // ROP/mode
        "SetROP2" => SetROP2(emu),
        "GetROP2" => GetROP2(emu),
        "SetStretchBltMode" => SetStretchBltMode(emu),
        "GetStretchBltMode" => GetStretchBltMode(emu),
        "SetPolyFillMode" => SetPolyFillMode(emu),
        "GetPolyFillMode" => GetPolyFillMode(emu),

        // Transform
        "SetWorldTransform" => SetWorldTransform(emu),
        "GetWorldTransform" => GetWorldTransform(emu),

        // Printer
        "StartDoc" | "StartDocA" | "StartDocW" => StartDoc(emu),
        "EndDoc" => EndDoc(emu),
        "StartPage" => StartPage(emu),
        "EndPage" => EndPage(emu),

        // Palette
        "GetSystemPaletteEntries" => GetSystemPaletteEntries(emu),
        "GetNearestColor" => GetNearestColor(emu),

        // DC colors
        "SetDCBrushColor" => SetDCBrushColor(emu),
        "SetDCPenColor" => SetDCPenColor(emu),

        // Layout
        "GetLayout" => GetLayout(emu),
        "SetLayout" => SetLayout(emu),

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented gdi32 API 0x{:x} {} at 0x{:x}",
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

            log::error!(
                "attempt to call unimplemented gdi32 API 0x{:x} {}",
                addr,
                api
            );
            log::warn!(
                "calling unimplemented gdi32 API 0x{:x} {} at 0x{:x}",
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

fn CreateDCA(emu: &mut emu::Emu) {
    let driver = arg(emu, 0) as u64;
    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        emu.maps.read_string(device)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!CreateDCA device:'{}'", name);
    emu.regs_mut().rax = 0xdc_0001;
    stdcall_ret(emu, 4);
}

fn CreateDCW(emu: &mut emu::Emu) {
    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        emu.maps.read_wide_string(device)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!CreateDCW device:'{}'", name);
    emu.regs_mut().rax = 0xdc_0001;
    stdcall_ret(emu, 4);
}

fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    emu.regs_mut().rax = 0xdc_0002;
    stdcall_ret(emu, 1);
}

fn DeleteDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", hdc);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn GetDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "gdi32!GetDC hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = 0xdc_0003;
    stdcall_ret(emu, 1);
}

fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1);
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SaveDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!SaveDC hdc:0x{:x}", hdc);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn RestoreDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let saved = arg(emu, 1) as i32;
    log_red!(emu, "gdi32!RestoreDC hdc:0x{:x} saved:{}", hdc, saved);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

// === Bitmap ===

fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let w = arg(emu, 1);
    let h = arg(emu, 2);
    log_red!(emu, "gdi32!CreateCompatibleBitmap {}x{}", w, h);
    emu.regs_mut().rax = 0xb0_0001;
    stdcall_ret(emu, 3);
}

fn CreateDIBSection(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateDIBSection");
    emu.regs_mut().rax = 0xb0_0002;
    stdcall_ret(emu, 6);
}

fn BitBlt(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!BitBlt");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

fn StretchBlt(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!StretchBlt");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 11);
}

fn PatBlt(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!PatBlt");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn SetDIBits(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetDIBits");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}

fn GetDIBits(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetDIBits");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 7);
}

fn GetDIBColorTable(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetDIBColorTable");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn SetBkMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetBkMode hdc:0x{:x} mode:{}", hdc, mode);
    emu.regs_mut().rax = 1; // previous mode
    stdcall_ret(emu, 2);
}

fn SetBkColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetBkColor color:0x{:06x}", color);
    emu.regs_mut().rax = 0x00ff_ffff; // previous color
    stdcall_ret(emu, 2);
}

// === Object ===

fn SelectObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj = arg(emu, 1);
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    emu.regs_mut().rax = obj as u64;
    stdcall_ret(emu, 2);
}

fn DeleteObject(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn GetObjectA(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    let size = arg(emu, 1);
    log_red!(emu, "gdi32!GetObjectA obj:0x{:x} size:{}", obj, size);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetObjectW(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    let size = arg(emu, 1);
    log_red!(emu, "gdi32!GetObjectW obj:0x{:x} size:{}", obj, size);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetObjectType(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x}", obj);
    emu.regs_mut().rax = 0; // OBJ_PEN=1, etc. — unknown
    stdcall_ret(emu, 1);
}

fn GetStockObject(emu: &mut emu::Emu) {
    let idx = arg(emu, 0);
    log_red!(emu, "gdi32!GetStockObject index:{}", idx);
    emu.regs_mut().rax = 0x1337_0000 + idx as u64;
    stdcall_ret(emu, 1);
}

fn GetCurrentObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj_type = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{}",
        hdc,
        obj_type
    );
    emu.regs_mut().rax = 0x1337_0000 + obj_type as u64;
    stdcall_ret(emu, 2);
}

fn GetDeviceCaps(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let index = arg(emu, 1) as i32;
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
    stdcall_ret(emu, 2);
}

// === Font ===

fn CreateFontIndirectA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontIndirectA");
    emu.regs_mut().rax = 0x1337_f001;
    stdcall_ret(emu, 1);
}

fn CreateFontIndirectW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontIndirectW");
    emu.regs_mut().rax = 0x1337_f002;
    stdcall_ret(emu, 1);
}

fn CreateFontA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontA");
    emu.regs_mut().rax = 0x1337_f003;
    stdcall_ret(emu, 14);
}

fn CreateFontW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateFontW");
    emu.regs_mut().rax = 0x1337_f004;
    stdcall_ret(emu, 14);
}

fn GetTextMetricsA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let tm = arg(emu, 1) as u64;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
    }
    log_red!(emu, "gdi32!GetTextMetricsA");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn GetTextMetricsW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let tm = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}

fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let len = arg(emu, 2);
    let size_ptr = arg(emu, 3) as u64;
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, len * 7); // cx
        emu.maps.write_dword(size_ptr + 4, 16); // cy
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32A len:{}", len);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let len = arg(emu, 2);
    let size_ptr = arg(emu, 3) as u64;
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, len * 7);
        emu.maps.write_dword(size_ptr + 4, 16);
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32W len:{}", len);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn GetTextFaceA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let count = arg(emu, 1);
    let buf = arg(emu, 2) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_string(buf, face);
    }
    log_red!(emu, "gdi32!GetTextFaceA => '{}'", face);
    emu.regs_mut().rax = face.len() as u64;
    stdcall_ret(emu, 3);
}

fn GetTextFaceW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let count = arg(emu, 1);
    let buf = arg(emu, 2) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_wide_string(buf, face);
    }
    log_red!(emu, "gdi32!GetTextFaceW => '{}'", face);
    emu.regs_mut().rax = face.len() as u64;
    stdcall_ret(emu, 3);
}

fn AddFontResourceExA(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "gdi32!AddFontResourceExA '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn AddFontResourceExW(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "gdi32!AddFontResourceExW '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!RemoveFontResourceExA");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!RemoveFontResourceExW");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn GetCharWidth32A(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + (i as u64) * 4, 8);
        }
    }
    log_red!(emu, "gdi32!GetCharWidth32A first:{} last:{}", first, last);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn GetCharWidth32W(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + (i as u64) * 4, 8);
        }
    }
    log_red!(emu, "gdi32!GetCharWidth32W first:{} last:{}", first, last);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn GetCharABCWidthsA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            let off = (i as u64) * 12; // ABC struct: 3 ints
            emu.maps.write_dword(buf + off, 1); // abcA
            emu.maps.write_dword(buf + off + 4, 6); // abcB
            emu.maps.write_dword(buf + off + 8, 1); // abcC
        }
    }
    log_red!(emu, "gdi32!GetCharABCWidthsA");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn GetCharABCWidthsW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            let off = (i as u64) * 12;
            emu.maps.write_dword(buf + off, 1);
            emu.maps.write_dword(buf + off + 4, 6);
            emu.maps.write_dword(buf + off + 8, 1);
        }
    }
    log_red!(emu, "gdi32!GetCharABCWidthsW");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!EnumFontFamiliesExA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!EnumFontFamiliesExW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

// === Text ===

fn TextOutA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_string(text_ptr);
    log_red!(emu, "gdi32!TextOutA '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn TextOutW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_wide_string(text_ptr);
    log_red!(emu, "gdi32!TextOutW '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn ExtTextOutA(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!ExtTextOutA");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}

fn ExtTextOutW(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!ExtTextOutW");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}

fn SetTextColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetTextColor color:0x{:06x}", color);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

fn GetTextColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!GetTextColor");
    emu.regs_mut().rax = 0; // black
    stdcall_ret(emu, 1);
}

fn SetTextAlign(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let align = arg(emu, 1);
    log_red!(emu, "gdi32!SetTextAlign align:0x{:x}", align);
    emu.regs_mut().rax = 0; // previous alignment
    stdcall_ret(emu, 2);
}

fn GetTextAlign(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!GetTextAlign");
    emu.regs_mut().rax = 0; // TA_LEFT | TA_TOP
    stdcall_ret(emu, 1);
}

// === Drawing ===

fn GdiRectangle(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Rectangle");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn GdiEllipse(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Ellipse");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

fn MoveToEx(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!MoveToEx");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn LineTo(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!LineTo");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn SetPixel(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetPixel");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn GetPixelGdi(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetPixel");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn FillRect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!FillRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn GdiPolygon(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Polygon");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn GdiPolyline(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Polyline");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn GdiPolyBezier(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!PolyBezier");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn GdiArc(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Arc");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

fn GdiPie(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Pie");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

fn GdiChord(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Chord");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

// === Region ===

fn CreateRectRgn(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateRectRgn");
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 4);
}

fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateRectRgnIndirect");
    emu.regs_mut().rax = 0x1337_0002;
    stdcall_ret(emu, 1);
}

fn GetClipBox(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let rect = arg(emu, 1) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "gdi32!GetClipBox");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 2);
}

fn CombineRgn(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CombineRgn");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 4);
}

fn SelectClipRgn(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SelectClipRgn");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 2);
}

fn IntersectClipRect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!IntersectClipRect");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 5);
}

fn ExcludeClipRect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!ExcludeClipRect");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 5);
}

// === Pen/Brush ===

fn CreatePen(emu: &mut emu::Emu) {
    let style = arg(emu, 0);
    let width = arg(emu, 1);
    let color = arg(emu, 2);
    log_red!(
        emu,
        "gdi32!CreatePen style:{} width:{} color:0x{:06x}",
        style,
        width,
        color
    );
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 3);
}

fn CreatePenIndirect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreatePenIndirect");
    emu.regs_mut().rax = 0x1337_0002;
    stdcall_ret(emu, 1);
}

fn CreateSolidBrush(emu: &mut emu::Emu) {
    let color = arg(emu, 0);
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:06x}", color);
    emu.regs_mut().rax = 0x1337_0b01;
    stdcall_ret(emu, 1);
}

fn CreateBrushIndirect(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreateBrushIndirect");
    emu.regs_mut().rax = 0x1337_0b02;
    stdcall_ret(emu, 1);
}

fn CreatePatternBrush(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!CreatePatternBrush");
    emu.regs_mut().rax = 0x1337_0b03;
    stdcall_ret(emu, 1);
}

fn CreateHatchBrush(emu: &mut emu::Emu) {
    let style = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    emu.regs_mut().rax = 0x1337_0b04;
    stdcall_ret(emu, 2);
}

// === Map/Viewport ===

fn SetMapMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetMapMode mode:{}", mode);
    emu.regs_mut().rax = 1; // previous mode: MM_TEXT
    stdcall_ret(emu, 2);
}

fn GetMapMode(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetMapMode");
    emu.regs_mut().rax = 1; // MM_TEXT
    stdcall_ret(emu, 1);
}

fn SetWindowOrgEx(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let prev = arg(emu, 3) as u64;
    if prev != 0 {
        emu.maps.write_dword(prev, 0);
        emu.maps.write_dword(prev + 4, 0);
    }
    log_red!(emu, "gdi32!SetWindowOrgEx x:{} y:{}", x, y);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn SetViewportOrgEx(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let prev = arg(emu, 3) as u64;
    if prev != 0 {
        emu.maps.write_dword(prev, 0);
        emu.maps.write_dword(prev + 4, 0);
    }
    log_red!(emu, "gdi32!SetViewportOrgEx x:{} y:{}", x, y);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

// === ROP/mode ===

fn SetROP2(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetROP2 mode:{}", mode);
    emu.regs_mut().rax = 13; // previous: R2_COPYPEN
    stdcall_ret(emu, 2);
}

fn GetROP2(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetROP2");
    emu.regs_mut().rax = 13; // R2_COPYPEN
    stdcall_ret(emu, 1);
}

fn SetStretchBltMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetStretchBltMode mode:{}", mode);
    emu.regs_mut().rax = 1; // previous mode
    stdcall_ret(emu, 2);
}

fn GetStretchBltMode(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetStretchBltMode");
    emu.regs_mut().rax = 1; // BLACKONWHITE
    stdcall_ret(emu, 1);
}

fn SetPolyFillMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetPolyFillMode mode:{}", mode);
    emu.regs_mut().rax = 1; // previous: ALTERNATE
    stdcall_ret(emu, 2);
}

fn GetPolyFillMode(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetPolyFillMode");
    emu.regs_mut().rax = 1; // ALTERNATE
    stdcall_ret(emu, 1);
}

// === Transform ===

fn SetWorldTransform(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!SetWorldTransform");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn GetWorldTransform(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let xform = arg(emu, 1) as u64;
    if xform != 0 {
        // Identity matrix: eM11=1.0, eM12=0, eM21=0, eM22=1.0, eDx=0, eDy=0
        emu.maps.write_dword(xform, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 4, 0);
        emu.maps.write_dword(xform + 8, 0);
        emu.maps.write_dword(xform + 12, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 16, 0);
        emu.maps.write_dword(xform + 20, 0);
    }
    log_red!(emu, "gdi32!GetWorldTransform");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

// === Printer ===

fn StartDoc(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!StartDoc");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn EndDoc(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!EndDoc");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn StartPage(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!StartPage");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn EndPage(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!EndPage");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

// === Palette ===

fn GetSystemPaletteEntries(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetSystemPaletteEntries");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn GetNearestColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!GetNearestColor color:0x{:06x}", color);
    emu.regs_mut().rax = color as u64;
    stdcall_ret(emu, 2);
}

// === DC colors ===

fn SetDCBrushColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetDCBrushColor color:0x{:06x}", color);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

fn SetDCPenColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetDCPenColor color:0x{:06x}", color);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

// === Layout ===

fn GetLayout(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!GetLayout");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn SetLayout(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let layout = arg(emu, 1);
    log_red!(emu, "gdi32!SetLayout layout:0x{:x}", layout);
    emu.regs_mut().rax = 0; // previous layout
    stdcall_ret(emu, 2);
}
