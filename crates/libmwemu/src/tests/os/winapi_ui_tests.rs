use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::system::*;

use crate::*;

// ─── GDI32 64-bit ───

#[test]
fn gdi32_64_create_compatible_dc() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(ret, 0, "CreateCompatibleDC should return non-zero handle");
}

#[test]
fn gdi32_64_create_compatible_bitmap() {
    helpers::setup();
    let mut emu = emu64();
    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::CreateCompatibleBitmap,
        &[hdc, 800, 600],
    );
    assert_ne!(
        ret, 0,
        "CreateCompatibleBitmap should return non-zero handle"
    );
}

#[test]
fn gdi32_64_select_delete_object() {
    helpers::setup();
    let mut emu = emu64();
    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    let brush = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateSolidBrush, &[0xFF0000]);
    let old = helpers::call_winapi64(&mut emu, winapi64::gdi32::SelectObject, &[hdc, brush]);
    assert_ne!(
        old, 0,
        "SelectObject returns non-zero previous object handle"
    );

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteObject, &[brush]);
    assert_eq!(ret, 1, "DeleteObject returns TRUE");
}

#[test]
fn gdi32_64_delete_dc() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteDC, &[0x1337_dc01]);
    assert_eq!(ret, 1, "DeleteDC returns TRUE");
}

#[test]
fn gdi32_64_bitblt() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::BitBlt,
        &[0xdc01, 0, 0, 800, 600, 0xdc02, 0, 0, SRCCOPY as u64],
    );
    assert_eq!(ret, 1, "BitBlt returns TRUE");
}

#[test]
fn gdi32_64_get_device_caps() {
    helpers::setup();
    let mut emu = emu64();

    let horzres = helpers::call_winapi64(&mut emu, winapi64::gdi32::GetDeviceCaps, &[0xdc01, 8]);
    assert_eq!(horzres, 1920, "HORZRES should be 1920");

    let vertres = helpers::call_winapi64(&mut emu, winapi64::gdi32::GetDeviceCaps, &[0xdc01, 10]);
    assert_eq!(vertres, 1080, "VERTRES should be 1080");

    let bitspixel = helpers::call_winapi64(&mut emu, winapi64::gdi32::GetDeviceCaps, &[0xdc01, 14]);
    assert_eq!(bitspixel, 32, "BITSPIXEL should be 32");

    let logpixelsx =
        helpers::call_winapi64(&mut emu, winapi64::gdi32::GetDeviceCaps, &[0xdc01, 88]);
    assert_eq!(logpixelsx, 96, "LOGPIXELSX should be 96");
}

#[test]
fn gdi32_64_create_font_a() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateFontA, &[]);
    assert_ne!(ret, 0, "CreateFontA should return a non-zero handle");
}

#[test]
fn gdi32_64_set_bk_mode_color() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::SetBkMode, &[0xdc01, 1]);
    assert_eq!(ret, 1, "SetBkMode returns previous mode (1)");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::SetBkColor, &[0xdc01, 0xFF0000]);
    assert_eq!(ret, 0x00ff_ffff, "SetBkColor returns previous color");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::SetTextColor, &[0xdc01, 0x00FF00]);
    assert_eq!(ret, 0, "SetTextColor returns previous color (0)");
}

#[test]
fn gdi32_64_stock_object() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::GetStockObject, &[13]);
    assert_ne!(ret, 0, "GetStockObject returns non-zero handle");
}

#[test]
fn gdi32_64_create_brush_pen() {
    helpers::setup();
    let mut emu = emu64();

    let brush = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateSolidBrush, &[0xFF0000]);
    assert_ne!(brush, 0, "CreateSolidBrush returns non-zero handle");

    let pen = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreatePen, &[0, 1, 0x00FF00]);
    assert_ne!(pen, 0, "CreatePen returns non-zero handle");
}

#[test]
fn gdi32_64_drawing_ops() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::MoveToEx, &[0xdc01, 10, 20, 0]);
    assert_eq!(ret, 1, "MoveToEx returns TRUE");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::LineTo, &[0xdc01, 100, 200]);
    assert_eq!(ret, 1, "LineTo returns TRUE");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::SetPixel, &[0xdc01, 50, 50, 0xFF]);
    assert_eq!(ret, 0xFF, "SetPixel returns the color that was set");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::GetPixelGdi, &[0xdc01, 50, 50]);
    assert_eq!(ret, 0, "GetPixel returns 0");
}

#[test]
fn gdi32_64_text_out_a() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x20000u64;
    emu.maps
        .create_map("strbuf", buf, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_string(buf, "Hello GDI");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::TextOutA,
        &[0xdc01, 10, 20, buf, 9],
    );
    assert_eq!(ret, 1, "TextOutA returns TRUE");
}

#[test]
fn gdi32_64_save_restore_dc() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(hdc, 0, "CreateCompatibleDC returns non-zero handle");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::SaveDC, &[hdc]);
    assert_eq!(ret, 1, "SaveDC returns 1");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::RestoreDC, &[hdc, 1]);
    assert_eq!(ret, 1, "RestoreDC returns TRUE");
}

// ─── GDI32 32-bit ───

#[test]
fn gdi32_32_create_compatible_dc() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(ret, 0, "32-bit CreateCompatibleDC returns non-zero handle");
}

#[test]
fn gdi32_32_create_compatible_bitmap() {
    helpers::setup();
    let mut emu = emu32();
    let hdc = helpers::call_winapi32(&mut emu, winapi32::gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::gdi32::CreateCompatibleBitmap,
        &[hdc, 640, 480],
    );
    assert_ne!(
        ret, 0,
        "32-bit CreateCompatibleBitmap returns non-zero handle"
    );
}

#[test]
fn gdi32_32_select_delete_object() {
    helpers::setup();
    let mut emu = emu32();
    let hdc = helpers::call_winapi32(&mut emu, winapi32::gdi32::CreateCompatibleDC, &[0]);
    let brush = helpers::call_winapi32(&mut emu, winapi32::gdi32::CreateSolidBrush, &[0xFF0000]);
    let old = helpers::call_winapi32(&mut emu, winapi32::gdi32::SelectObject, &[hdc, brush]);
    assert_ne!(
        old, 0,
        "32-bit SelectObject returns non-zero previous object"
    );

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::DeleteObject, &[brush]);
    assert_eq!(ret, 1, "32-bit DeleteObject returns TRUE");
}

#[test]
fn gdi32_32_get_device_caps() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::GetDeviceCaps, &[0xdc_0002, 8]);
    assert_eq!(ret, 1920, "32-bit HORZRES should be 1920");

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::GetDeviceCaps, &[0xdc_0002, 10]);
    assert_eq!(ret, 1080, "32-bit VERTRES should be 1080");
}

#[test]
fn gdi32_32_drawing_ops() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::MoveToEx, &[0xdc_0002, 10, 20, 0]);
    assert_eq!(ret, 1, "32-bit MoveToEx returns TRUE");

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::LineTo, &[0xdc_0002, 100, 200]);
    assert_eq!(ret, 1, "32-bit LineTo returns TRUE");
}

#[test]
fn gdi32_32_bitblt() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::gdi32::BitBlt,
        &[0xdc01, 0, 0, 800, 600, 0xdc02, 0, 0, SRCCOPY],
    );
    assert_eq!(ret, 1, "32-bit BitBlt returns TRUE");
}

#[test]
fn gdi32_32_save_restore_dc() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::SaveDC, &[0xdc_0002]);
    assert_eq!(ret, 1, "32-bit SaveDC returns 1");

    let ret = helpers::call_winapi32(&mut emu, winapi32::gdi32::RestoreDC, &[0xdc_0002, 1]);
    assert_eq!(ret, 1, "32-bit RestoreDC returns TRUE");
}

#[test]
fn gdi32_32_create_font_a() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::gdi32::CreateFontA,
        &[16, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    );
    assert_ne!(ret, 0, "32-bit CreateFontA should return a non-zero handle");
}

// ─── Shell32 64-bit ───

#[test]
fn shell32_64_shell_execute_a() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x20000u64;
    emu.maps
        .create_map("strbuf", buf, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_string(buf, "open");
    emu.maps.write_string(buf + 0x100, "notepad.exe");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::shell32::ShellExecuteA,
        &[0, buf, buf + 0x100, 0, 0, 1],
    );
    assert!(
        ret > 32,
        "ShellExecuteA should return >32 on success, got {}",
        ret
    );
}

#[test]
fn shell32_64_shgetfolderpath_w() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x20000u64;
    emu.maps
        .create_map("pathbuf", buf, 0x2000, Permission::READ_WRITE)
        .unwrap();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::shell32::SHGetFolderPathW,
        &[0, CSIDL_WINDOWS as u64, 0, 0, buf],
    );
    assert_eq!(ret, 0, "SHGetFolderPathW returns S_OK");

    let path = emu.maps.read_wide_string(buf);
    assert!(
        path.contains("Windows"),
        "SHGetFolderPathW should write a Windows path, got '{}'",
        path
    );
}

// ─── Shell32 32-bit ───

#[test]
fn shell32_32_shell_execute_a() {
    helpers::setup();
    let mut emu = emu32();

    let buf = 0x20000u64;
    emu.maps
        .create_map("strbuf", buf, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_string(buf, "open");
    emu.maps.write_string(buf + 0x100, "calc.exe");

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::shell32::ShellExecuteA,
        &[0, buf as u32, (buf + 0x100) as u32, 0, 0, 1],
    );
    assert!(
        ret > 32,
        "32-bit ShellExecuteA should return >32 on success, got {}",
        ret
    );
}

// ─── GDI32 full pipeline (create, draw, cleanup) ───

#[test]
fn gdi32_64_full_pipeline() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(hdc, 0);

    let hbmp = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::CreateCompatibleBitmap,
        &[hdc, 1024, 768],
    );
    assert_ne!(hbmp, 0);

    let old_obj = helpers::call_winapi64(&mut emu, winapi64::gdi32::SelectObject, &[hdc, hbmp]);
    assert_ne!(old_obj, 0, "SelectObject returns non-zero previous object");

    helpers::call_winapi64(&mut emu, winapi64::gdi32::SaveDC, &[hdc]);

    let brush = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateSolidBrush, &[0xFF0000]);
    assert_ne!(brush, 0);

    let pen = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreatePen, &[0, 2, 0x00FF00]);
    assert_ne!(pen, 0);

    helpers::call_winapi64(&mut emu, winapi64::gdi32::MoveToEx, &[hdc, 0, 0, 0]);
    helpers::call_winapi64(&mut emu, winapi64::gdi32::LineTo, &[hdc, 100, 100]);

    let blt = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::BitBlt,
        &[hdc, 0, 0, 1024, 768, hdc, 0, 0, SRCCOPY as u64],
    );
    assert_eq!(blt, 1);

    helpers::call_winapi64(&mut emu, winapi64::gdi32::RestoreDC, &[hdc, 1]);

    let del_brush = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteObject, &[brush]);
    assert_eq!(del_brush, 1);

    let del_pen = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteObject, &[pen]);
    assert_eq!(del_pen, 1);

    let del_bmp = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteObject, &[hbmp]);
    assert_eq!(del_bmp, 1);

    let del_dc = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteDC, &[hdc]);
    assert_eq!(del_dc, 1);
}

#[test]
fn message_box_w_64() {
    helpers::setup();
    let mut emu = emu64();

    let text_addr = 0x100000u64;
    let caption_addr = 0x100100u64;
    emu.maps
        .create_map("msgbox_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"Alert" as caption
    let caption: [u8; 12] = [b'A', 0, b'l', 0, b'e', 0, b'r', 0, b't', 0, 0, 0];
    for (i, &b) in caption.iter().enumerate() {
        emu.maps.write_byte(caption_addr + i as u64, b);
    }
    // Write L"OK" as text
    let text: [u8; 6] = [b'O', 0, b'K', 0, 0, 0];
    for (i, &b) in text.iter().enumerate() {
        emu.maps.write_byte(text_addr + i as u64, b);
    }

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::user32::MessageBoxW,
        &[0, text_addr, caption_addr, 0],
    );
    assert_eq!(ret, 1, "MessageBoxW must return IDOK (1)");
}

#[test]
fn find_window_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let class_addr = 0x100000u64;
    emu.maps
        .create_map("fw_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(class_addr, "Notepad");

    let ret = helpers::call_winapi64(&mut emu, winapi64::user32::FindWindowA, &[class_addr, 0]);
    assert_eq!(ret, 0, "FindWindowA must return NULL (window not found)");
}

#[test]
fn get_key_state_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::user32::GetKeyState, &[b'A' as u64]);
    assert_eq!(ret, 0, "GetKeyState must return 0 (not pressed)");

    let ret = helpers::call_winapi64(&mut emu, winapi64::user32::GetAsyncKeyState, &[0x41]);
    assert_eq!(ret, 0, "GetAsyncKeyState must return 0");
}

#[test]
fn set_windows_hook_ex_a_64() {
    helpers::setup();
    let mut emu = emu64();

    // WH_KEYBOARD_LL = 13
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::user32::SetWindowsHookExA,
        &[13, 0x401000, 0, 0],
    );
    assert_ne!(ret, 0, "SetWindowsHookExA must return a hook handle");
}
