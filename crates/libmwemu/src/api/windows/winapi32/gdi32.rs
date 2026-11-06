use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
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

        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

// === DC operations ===

pub fn CreateDCA(emu: &mut emu::Emu) {
    let driver = arg(emu, 0) as u64;
    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        emu.maps.read_string(device)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!CreateDCA device:'{}'", name);
    let hndl = helper::handler_create("gdi32://CreateDCA");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        emu.maps.read_wide_string(device)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!CreateDCW device:'{}'", name);
    let hndl = helper::handler_create("gdi32://CreateDCW");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}

pub fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    let hndl = helper::handler_create("gdi32://CreateCompatibleDC");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}

pub fn DeleteDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn GetDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "gdi32!GetDC hwnd:0x{:x}", hwnd);
    let hndl = helper::handler_create("gdi32://GetDC");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}

pub fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1) as u64;
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn SaveDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!SaveDC hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn RestoreDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let saved = arg(emu, 1) as i32;
    log_red!(emu, "gdi32!RestoreDC hdc:0x{:x} saved:{}", hdc, saved);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

// === Bitmap ===

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let w = arg(emu, 1);
    let h = arg(emu, 2);
    log_red!(emu, "gdi32!CreateCompatibleBitmap {}x{}", w, h);
    let hndl = helper::handler_create("gdi32://CreateCompatibleBitmap");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 3);
}

pub fn CreateDIBSection(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!CreateDIBSection arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateDIBSection");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn BitBlt(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!BitBlt arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn StretchBlt(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!StretchBlt arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..11u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PatBlt(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!PatBlt arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetDIBits(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!SetDIBits arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetDIBits(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GetDIBits arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetDIBColorTable(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GetDIBColorTable arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetBkMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetBkMode hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode
    stdcall_ret(emu, 2);
}

pub fn SetBkColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetBkColor color:0x{:06x}", color);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00ff_ffff; // previous color
    stdcall_ret(emu, 2);
}

// === Object ===

pub fn SelectObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj = arg(emu, 1);
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    let prev = helper::handler_create("gdi32://prev_object");
    assert!(prev < 0x00000001_00000000);
    emu.regs_mut().rax = prev;
    stdcall_ret(emu, 2);
}

pub fn DeleteObject(emu: &mut emu::Emu) {
    let obj = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::handler_close(obj);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn GetObjectA(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    let size = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    log_red!(
        emu,
        "gdi32!GetObjectA obj:0x{:x} size:{} buf:0x{:x}",
        obj,
        size,
        buffer
    );
    if buffer != 0 && size > 0 {
        let fill = size.min(64) as usize;
        for i in 0..fill {
            emu.maps.write_byte(buffer + i as u64, 0);
        }
        emu.regs_mut().rax = fill as u64;
    } else {
        helper::advance_tick(emu, 1);
        emu.regs_mut().rax = 24;
    }
    stdcall_ret(emu, 3);
}

pub fn GetObjectW(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    let size = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    log_red!(
        emu,
        "gdi32!GetObjectW obj:0x{:x} size:{} buf:0x{:x}",
        obj,
        size,
        buffer
    );
    if buffer != 0 && size > 0 {
        let fill = size.min(64) as usize;
        for i in 0..fill {
            emu.maps.write_byte(buffer + i as u64, 0);
        }
        emu.regs_mut().rax = fill as u64;
    } else {
        helper::advance_tick(emu, 1);
        emu.regs_mut().rax = 24;
    }
    stdcall_ret(emu, 3);
}

pub fn GetObjectType(emu: &mut emu::Emu) {
    let obj = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::advance_tick(emu, 1);
        emu.set_last_error(0);
        emu.regs_mut().rax = 6; // OBJ_BITMAP
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 1);
}

pub fn GetStockObject(emu: &mut emu::Emu) {
    let idx = arg(emu, 0);
    log_red!(emu, "gdi32!GetStockObject index:{}", idx);
    let hndl = helper::handler_create("gdi32://stock_object");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj_type = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{}",
        hdc,
        obj_type
    );
    let hndl = helper::handler_create("gdi32://current_object");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 2);
}

pub fn GetDeviceCaps(emu: &mut emu::Emu) {
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
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 2);
}

// === Font ===

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreateFontIndirectA arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateFontIndirectA");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreateFontIndirectW arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateFontIndirectW");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateFontA(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!CreateFontA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..14u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateFontA");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!CreateFontW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..14u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateFontW");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn GetTextMetricsA(emu: &mut emu::Emu) {
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetTextMetricsA arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetTextMetricsW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
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

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
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

pub fn GetTextFaceA(emu: &mut emu::Emu) {
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

pub fn GetTextFaceW(emu: &mut emu::Emu) {
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

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "gdi32!AddFontResourceExA '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "gdi32!AddFontResourceExW '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn GetCharWidth32A(emu: &mut emu::Emu) {
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

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
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

pub fn GetCharABCWidthsA(emu: &mut emu::Emu) {
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn GetCharABCWidthsW(emu: &mut emu::Emu) {
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetCharABCWidthsW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!EnumFontFamiliesExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!EnumFontFamiliesExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Text ===

pub fn TextOutA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_string(text_ptr);
    log_red!(emu, "gdi32!TextOutA '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn TextOutW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_wide_string(text_ptr);
    log_red!(emu, "gdi32!TextOutW '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn ExtTextOutA(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!ExtTextOutA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..8u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!ExtTextOutW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..8u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetTextColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetTextColor color:0x{:06x}", color);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

pub fn GetTextColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!GetTextColor arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // black
    stdcall_ret(emu, 1);
}

pub fn SetTextAlign(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let align = arg(emu, 1);
    log_red!(emu, "gdi32!SetTextAlign align:0x{:x}", align);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous alignment
    stdcall_ret(emu, 2);
}

pub fn GetTextAlign(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!GetTextAlign arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // TA_LEFT | TA_TOP
    stdcall_ret(emu, 1);
}

// === Drawing ===

pub fn GdiRectangle(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Rectangle arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn GdiEllipse(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GdiEllipse arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn MoveToEx(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!MoveToEx arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LineTo(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!LineTo arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetPixel(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!SetPixel arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetPixelGdi(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GetPixelGdi arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FillRect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!FillRect arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GdiPolygon(emu: &mut emu::Emu) {
    log_red!(emu, "gdi32!Polygon");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn GdiPolyline(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Polyline arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn GdiPolyBezier(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!PolyBezier arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn GdiArc(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    let arg7 = emu.maps.read_dword(esp + 28).unwrap_or(0) as u64;
    let arg8 = emu.maps.read_dword(esp + 32).unwrap_or(0) as u64;
    let arg9 = emu.maps.read_dword(esp + 36).unwrap_or(0) as u64;
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Arc arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x} arg8:0x{:x} arg9:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

pub fn GdiPie(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    let arg7 = emu.maps.read_dword(esp + 28).unwrap_or(0) as u64;
    let arg8 = emu.maps.read_dword(esp + 32).unwrap_or(0) as u64;
    let arg9 = emu.maps.read_dword(esp + 36).unwrap_or(0) as u64;
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Pie arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x} arg8:0x{:x} arg9:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

pub fn GdiChord(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    let arg7 = emu.maps.read_dword(esp + 28).unwrap_or(0) as u64;
    let arg8 = emu.maps.read_dword(esp + 32).unwrap_or(0) as u64;
    let arg9 = emu.maps.read_dword(esp + 36).unwrap_or(0) as u64;
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!Chord arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x} arg8:0x{:x} arg9:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}

// === Region ===

pub fn CreateRectRgn(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!CreateRectRgn arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateRectRgn");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreateRectRgnIndirect arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateRectRgnIndirect");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn GetClipBox(emu: &mut emu::Emu) {
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

pub fn CombineRgn(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!CombineRgn arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SelectClipRgn(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!SelectClipRgn arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn IntersectClipRect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!IntersectClipRect arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ExcludeClipRect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!ExcludeClipRect arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Pen/Brush ===

pub fn CreatePen(emu: &mut emu::Emu) {
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
    let hndl = helper::handler_create("gdi32://CreatePen");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 3);
}

pub fn CreatePenIndirect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreatePenIndirect arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreatePenIndirect");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateSolidBrush(emu: &mut emu::Emu) {
    let color = arg(emu, 0);
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:06x}", color);
    let hndl = helper::handler_create("gdi32://CreateSolidBrush");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}

pub fn CreateBrushIndirect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreateBrushIndirect arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateBrushIndirect");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreatePatternBrush(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreatePatternBrush arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreatePatternBrush");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateHatchBrush(emu: &mut emu::Emu) {
    let style = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    let hndl = helper::handler_create("gdi32://CreateHatchBrush");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 2);
}

// === Map/Viewport ===

pub fn SetMapMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetMapMode mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode: MM_TEXT
    stdcall_ret(emu, 2);
}

pub fn GetMapMode(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!GetMapMode arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetWindowOrgEx(emu: &mut emu::Emu) {
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

pub fn SetViewportOrgEx(emu: &mut emu::Emu) {
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

pub fn SetROP2(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetROP2 mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 13; // previous: R2_COPYPEN
    stdcall_ret(emu, 2);
}

pub fn GetROP2(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!GetROP2 arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetStretchBltMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetStretchBltMode mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode
    stdcall_ret(emu, 2);
}

pub fn GetStretchBltMode(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!GetStretchBltMode arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetPolyFillMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetPolyFillMode mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous: ALTERNATE
    stdcall_ret(emu, 2);
}

pub fn GetPolyFillMode(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!GetPolyFillMode arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

// === Transform ===

pub fn SetWorldTransform(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0) as u64;
    let lpxform = arg(emu, 1) as u64;
    log_red!(
        emu,
        "gdi32!SetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        lpxform
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn GetWorldTransform(emu: &mut emu::Emu) {
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "gdi32!GetWorldTransform arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

// === Printer ===

pub fn StartDoc(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!StartDoc arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn EndDoc(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!EndDoc arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn StartPage(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!StartPage arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn EndPage(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!EndPage arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

// === Palette ===

pub fn GetSystemPaletteEntries(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GetSystemPaletteEntries arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetNearestColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!GetNearestColor color:0x{:06x}", color);
    emu.set_last_error(0);
    emu.regs_mut().rax = color as u64;
    stdcall_ret(emu, 2);
}

// === DC colors ===

pub fn SetDCBrushColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetDCBrushColor color:0x{:06x}", color);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

pub fn SetDCPenColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetDCPenColor color:0x{:06x}", color);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}

// === Layout ===

pub fn GetLayout(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!GetLayout arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetLayout(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let layout = arg(emu, 1);
    log_red!(emu, "gdi32!SetLayout layout:0x{:x}", layout);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous layout
    stdcall_ret(emu, 2);
}
