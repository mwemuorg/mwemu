use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::libraries::gdi32;
use crate::winapi::window_table::Window;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::controls::*;
use crate::windows::constants::errors::{
    E_NOTIMPL, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, RPC_E_CALL_COMPLETE,
    STATUS_INVALID_HANDLE,
};
use crate::windows::constants::system::*;
use crate::*;

// ─── GDI32 64-bit ───

#[test]
fn gdi32_64_create_compatible_dc() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(ret, 0, "CreateCompatibleDC should return non-zero handle");
}

#[test]
fn gdi32_64_create_compatible_bitmap() {
    helpers::setup();
    let mut emu = emu64();
    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleBitmap, &[hdc, 800, 600]);
    assert_ne!(
        ret, 0,
        "CreateCompatibleBitmap should return non-zero handle"
    );
}

#[test]
fn gdi32_64_select_delete_object() {
    helpers::setup();
    let mut emu = emu64();
    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let brush = helpers::call_winapi64(&mut emu, gdi32::CreateSolidBrush, &[0xFF0000]);
    let old = helpers::call_winapi64(&mut emu, gdi32::SelectObject, &[hdc, brush]);
    assert_ne!(
        old, 0,
        "SelectObject returns non-zero previous object handle"
    );

    helpers::call_winapi64(&mut emu, gdi32::SelectObject, &[hdc, old]);
    let ret = helpers::call_winapi64(&mut emu, gdi32::DeleteObject, &[brush]);
    assert_eq!(ret, 1, "DeleteObject returns TRUE");
}

#[test]
fn gdi32_64_delete_dc() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, gdi32::DeleteDC, &[0x1337_dc01]);
    assert_eq!(ret, 0, "DeleteDC rejects an unknown handle");
}

#[test]
fn bitblt_rejects_an_unknown_destination_and_reports_unmodeled_pixels() {
    use crate::ApiMode;
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::{ERROR_CALL_NOT_IMPLEMENTED, ERROR_INVALID_HANDLE};
    for is64 in [false, true] {
        let (mut emu, _) = prepare_winapi(is64);
        let mut args = [0, 0, 0, 800, 600, 0, 0, 0, SRCCOPY.into()];
        assert_eq!(call(&mut emu, "gdi32.dll", "BitBlt", &args), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        let dc = call(&mut emu, "user32.dll", "GetDC", &[0]);
        args[0] = dc;
        args[5] = dc;
        emu.set_api_mode(ApiMode::SimulateFail);
        assert_eq!(call(&mut emu, "gdi32.dll", "BitBlt", &args), 0);
        assert_eq!(emu.get_last_error(), ERROR_CALL_NOT_IMPLEMENTED);
    }
}

#[test]
fn gdi32_64_create_font_a() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, gdi32::CreateFontA, &[]);
    assert_ne!(ret, 0, "CreateFontA should return a non-zero handle");
}

#[test]
fn dc_attributes_follow_the_handle_and_saved_level() {
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        let mut gdi =
            |api: &str, args: &[u64]| helpers::call_winapi_export(&mut emu, "gdi32.dll", api, args);
        let h = gdi("CreateCompatibleDC", &[0]);
        assert_ne!(h, 0);
        assert_eq!(gdi("GetBkColor", &[h]), 0x00ff_ffff);
        assert_eq!(gdi("SaveDC", &[h]), 1);
        assert_eq!(gdi("SetBkColor", &[h, 0x123456]), 0x00ff_ffff);
        assert_eq!(gdi("GetBkColor", &[h]), 0x123456);
        assert_eq!(gdi("SetTextColor", &[h, 0x99]), 0);
        assert_eq!(gdi("SetBkMode", &[h, 1]), 2);
        assert_eq!(gdi("RestoreDC", &[h, u64::MAX]), 1);
        assert_eq!(gdi("GetBkColor", &[h]), 0x00ff_ffff);
        assert_eq!(gdi("GetTextColor", &[h]), 0);
        assert_eq!(gdi("DeleteDC", &[h]), 1);
        assert_eq!(gdi("GetBkColor", &[h]), u32::MAX as u64);
    }
}

#[test]
fn gdi32_64_stock_object() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, gdi32::GetStockObject, &[13]);
    assert_ne!(ret, 0, "GetStockObject returns non-zero handle");
}

#[test]
fn gdi32_64_create_brush_pen() {
    helpers::setup();
    let mut emu = emu64();

    let brush = helpers::call_winapi64(&mut emu, gdi32::CreateSolidBrush, &[0xFF0000]);
    assert_ne!(brush, 0, "CreateSolidBrush returns non-zero handle");

    let pen = helpers::call_winapi64(&mut emu, gdi32::CreatePen, &[0, 1, 0x00FF00]);
    assert_ne!(pen, 0, "CreatePen returns non-zero handle");
}

#[test]
fn dc_position_and_modes_round_trip_through_save_restore() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let lib = "gdi32.dll";
        let dc = call(&mut emu, lib, "CreateCompatibleDC", &[0]);
        assert_ne!(dc, 0);
        assert_eq!(call(&mut emu, lib, "MoveToEx", &[0, 10, 20, data]), 0);
        assert_eq!(call(&mut emu, lib, "MoveToEx", &[dc, 10, 20, data]), 1);
        assert_eq!(emu.maps.read_qword(data), Some(0));
        assert_eq!(call(&mut emu, lib, "SaveDC", &[dc]), 1);
        assert_eq!(call(&mut emu, lib, "MoveToEx", &[dc, 15, 30, data]), 1);
        assert_eq!(emu.maps.read_dword(data), Some(10));
        assert_eq!(emu.maps.read_dword(data + 4), Some(20));
        assert_eq!(call(&mut emu, lib, "SetMapMode", &[dc, 8]), 1);
        assert_eq!(call(&mut emu, lib, "GetMapMode", &[dc]), 8);
        assert_eq!(call(&mut emu, lib, "SetMapMode", &[dc, 0]), 0);
        assert_eq!(call(&mut emu, lib, "RestoreDC", &[dc, 1]), 1);
        assert_eq!(call(&mut emu, lib, "GetMapMode", &[dc]), 1);
        assert_eq!(call(&mut emu, lib, "MoveToEx", &[dc, 1, 2, data]), 1);
        assert_eq!(emu.maps.read_dword(data), Some(10));
    }
}

#[test]
fn drawing_with_an_invalid_dc_returns_the_export_specific_error() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for (api, args, result) in [
            ("TextOutA", vec![0, 0, 0, data, 1], 0),
            ("TextOutW", vec![0, 0, 0, data, 1], 0),
            ("GetPixel", vec![0, 0, 0], u64::from(u32::MAX)),
            ("SetPixel", vec![0, 0, 0, 0], u64::from(u32::MAX)),
        ] {
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "gdi32.dll", api, &args),
                result,
                "{api}"
            );
        }
    }
}

#[test]
fn gdi32_64_save_restore_dc() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(hdc, 0, "CreateCompatibleDC returns non-zero handle");

    let ret = helpers::call_winapi64(&mut emu, gdi32::SaveDC, &[hdc]);
    assert_eq!(ret, 1, "SaveDC returns 1");

    let ret = helpers::call_winapi64(&mut emu, gdi32::RestoreDC, &[hdc, 1]);
    assert_eq!(ret, 1, "RestoreDC returns TRUE");
}

// ─── GDI32 32-bit ───

#[test]
fn gdi32_32_create_compatible_dc() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(ret, 0, "32-bit CreateCompatibleDC returns non-zero handle");
}

#[test]
fn gdi32_32_create_compatible_bitmap() {
    helpers::setup();
    let mut emu = emu32();
    let hdc = helpers::call_winapi32(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi32(&mut emu, gdi32::CreateCompatibleBitmap, &[hdc, 640, 480]);
    assert_ne!(
        ret, 0,
        "32-bit CreateCompatibleBitmap returns non-zero handle"
    );
}

#[test]
fn gdi32_32_select_delete_object() {
    helpers::setup();
    let mut emu = emu32();
    let hdc = helpers::call_winapi32(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let brush = helpers::call_winapi32(&mut emu, gdi32::CreateSolidBrush, &[0xFF0000]);
    let old = helpers::call_winapi32(&mut emu, gdi32::SelectObject, &[hdc, brush]);
    assert_ne!(
        old, 0,
        "32-bit SelectObject returns non-zero previous object"
    );

    helpers::call_winapi32(&mut emu, gdi32::SelectObject, &[hdc, old]);
    let ret = helpers::call_winapi32(&mut emu, gdi32::DeleteObject, &[brush]);
    assert_eq!(ret, 1, "32-bit DeleteObject returns TRUE");
}

#[test]
fn window_and_display_dcs_keep_their_matching_release_operations() {
    for wide in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(wide);
        const UNKNOWN_DC: u64 = 0x1234_0001;
        const LAST_SAVED_DC: u64 = (-1_i32) as u64;
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "gdi32.dll",
                "RestoreDC",
                &[UNKNOWN_DC, LAST_SAVED_DC]
            ),
            0
        );
        let hwnd = create_test_window(&mut emu, "paint test");
        for (name, args) in [
            ("GetDC", vec![hwnd]),
            ("GetWindowDC", vec![hwnd]),
            ("GetDCEx", vec![hwnd, 0, 0]),
        ] {
            let dc = helpers::call_winapi_export(&mut emu, "user32.dll", name, &args);
            assert_ne!(dc, 0, "{name}");
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "gdi32.dll", "SaveDC", &[dc]),
                1
            );
            for expected in [1, 0] {
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "gdi32.dll",
                        "RestoreDC",
                        &[dc, LAST_SAVED_DC]
                    ),
                    expected
                );
            }
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "gdi32.dll", "DeleteDC", &[dc]),
                0
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "user32.dll", "ReleaseDC", &[hwnd, dc]),
                1
            );
        }
        let dc = helpers::call_winapi_export(&mut emu, "user32.dll", "BeginPaint", &[hwnd, data]);
        assert_ne!(dc, 0);
        assert_eq!(emu.maps.read_dword(data), Some(dc as u32));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "user32.dll", "EndPaint", &[hwnd, data]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "gdi32.dll", "SaveDC", &[dc]),
            0
        );
        emu.guest_windows.destroy(hwnd);
    }
}

#[test]
fn gdi32_32_create_font_a() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(
        &mut emu,
        gdi32::CreateFontA,
        &[16, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    );
    assert_ne!(ret, 0, "32-bit CreateFontA should return a non-zero handle");
}

// ─── Shell32 64-bit ───

// ─── Shell32 32-bit ───

// ─── GDI32 full pipeline (create, draw, cleanup) ───

#[test]
fn display_objects_and_saved_dc_can_be_released() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(hdc, 0);

    let hbmp = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleBitmap, &[hdc, 1024, 768]);
    assert_ne!(hbmp, 0);

    let old_obj = helpers::call_winapi64(&mut emu, gdi32::SelectObject, &[hdc, hbmp]);
    assert_ne!(old_obj, 0, "SelectObject returns non-zero previous object");

    helpers::call_winapi64(&mut emu, gdi32::SaveDC, &[hdc]);

    let brush = helpers::call_winapi64(&mut emu, gdi32::CreateSolidBrush, &[0xFF0000]);
    assert_ne!(brush, 0);

    let pen = helpers::call_winapi64(&mut emu, gdi32::CreatePen, &[0, 2, 0x00FF00]);
    assert_ne!(pen, 0);

    helpers::call_winapi64(&mut emu, gdi32::MoveToEx, &[hdc, 0, 0, 0]);
    helpers::call_winapi64(&mut emu, gdi32::RestoreDC, &[hdc, 1]);

    let del_brush = helpers::call_winapi64(&mut emu, gdi32::DeleteObject, &[brush]);
    assert_eq!(del_brush, 1);

    let del_pen = helpers::call_winapi64(&mut emu, gdi32::DeleteObject, &[pen]);
    assert_eq!(del_pen, 1);

    helpers::call_winapi64(&mut emu, gdi32::SelectObject, &[hdc, old_obj]);
    let del_bmp = helpers::call_winapi64(&mut emu, gdi32::DeleteObject, &[hbmp]);
    assert_eq!(del_bmp, 1);

    let del_dc = helpers::call_winapi64(&mut emu, gdi32::DeleteDC, &[hdc]);
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
fn keyboard_state_updates_only_the_guest_thread_buffer() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let lib = "user32.dll";
        emu.maps.fill_range(data, 0, 256);
        emu.maps.write_byte(data + 65, 0x81);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, lib, "SetKeyboardState", &[data]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, lib, "GetKeyState", &[65]),
            0x8001
        );
        emu.maps.fill_range(data, 0xee, 257);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, lib, "GetKeyboardState", &[data]),
            1
        );
        assert_eq!(emu.maps.read_byte(data + 65), Some(0x81));
        assert_eq!(emu.maps.read_byte(data + 256), Some(0xee));
    }
}

#[test]
fn virtual_key_variants_share_the_us_keyboard_profile() {
    const LAST_ERROR: u64 = 0xcccc;
    // B and Z catch the old linear A-Z scan-code approximation.
    let cases = [
        (b'B' as u32, MAPVK_VK_TO_VSC, 0x30),
        (b'Z' as u32, MAPVK_VK_TO_VSC_EX, 0x2c),
        (0x30, MAPVK_VSC_TO_VK, b'B' as u32),
        (0x2c, MAPVK_VSC_TO_VK_EX, b'Z' as u32),
        (b'A' as u32, MAPVK_VK_TO_CHAR, b'A' as u32),
        (VK_RETURN, MAPVK_VK_TO_VSC, 0x1c),
        (u32::MAX, MAPVK_VK_TO_VSC, 0),
        (b'A' as u32, u32::MAX, 0),
    ];
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        for (api, extended) in [
            ("MapVirtualKeyA", false),
            ("MapVirtualKeyW", false),
            ("MapVirtualKeyExA", true),
            ("MapVirtualKeyExW", true),
        ] {
            for (code, kind, expected) in cases {
                let args = [code as u64, kind as u64, 0];
                emu.set_last_error(LAST_ERROR);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "user32.dll",
                        api,
                        &args[..if extended { 3 } else { 2 }]
                    ),
                    expected as u64,
                    "{api}: {code:#x}, {kind}"
                );
                assert_eq!(emu.get_last_error(), LAST_ERROR);
            }
        }
    }
}

#[test]
fn shared_libraries_dispatch_named_imports_and_mapped_exports() {
    for is64 in [false, true] {
        for by_import in [true, false] {
            let (mut emu, data) = helpers::prepare_winapi(is64);
            let invoke = |emu: &mut emu::Emu, dll: &str, api: &str, args: &[u64]| {
                if !by_import {
                    return helpers::call_winapi_export(emu, dll, api, args);
                }
                let sp = emu.regs().rsp;
                let result = if is64 {
                    helpers::call_winapi64(
                        emu,
                        |emu| winapi64::gateway_by_import(emu, dll, api, 0),
                        args,
                    )
                } else {
                    helpers::call_winapi32(
                        emu,
                        |emu| winapi32::gateway_by_import(emu, dll, api, 0),
                        &args.iter().map(|arg| *arg as u32).collect::<Vec<_>>(),
                    ) as u64
                };
                assert_eq!(emu.regs().rsp, sp);
                assert!(!emu.api_policy.stopped(), "{dll}!{api}");
                result
            };
            // A fresh emulator has no entry in the export index on the named-import pass.
            let desktop = invoke(&mut emu, "user32.dll", "GetDesktopWindow", &[]);
            assert_ne!(desktop, 0);
            assert_eq!(invoke(&mut emu, "user32.dll", "IsWindow", &[desktop]), 1);
            assert_eq!(
                invoke(&mut emu, "ole32.dll", "CoTestCancel", &[]),
                RPC_E_CALL_COMPLETE
            );
            let bstr = invoke(&mut emu, "oleaut32.dll", "SysAllocStringLen", &[0, 3]);
            assert_ne!(bstr, 0);
            assert_eq!(
                invoke(&mut emu, "oleaut32.dll", "SysStringByteLen", &[bstr]),
                6
            );
            invoke(&mut emu, "oleaut32.dll", "SysFreeString", &[bstr]);
            assert_eq!(
                invoke(&mut emu, "crypt32.dll", "CertFreeCertificateContext", &[0]),
                1
            );
            assert_eq!(
                invoke(&mut emu, "bcrypt.dll", "BCryptGenRandom", &[0, data, 8, 0]),
                STATUS_INVALID_HANDLE
            );
            for (dll, api) in [
                ("cryptbase.dll", "SystemFunction036"),
                ("bcryptprimitives.dll", "ProcessPrng"),
            ] {
                assert_eq!(invoke(&mut emu, dll, api, &[data, 8]), 1);
                assert_eq!(invoke(&mut emu, dll, api, &[0, 8]), 0);
            }
            emu.maps.write_qword(data, u64::MAX);
            assert_eq!(
                invoke(&mut emu, "winmm.dll", "timeGetDevCaps", &[data, 8]),
                0
            );
            assert_eq!(emu.maps.read_dword(data), Some(1));
            assert_eq!(emu.maps.read_dword(data + 4), Some(1_000_000));
            assert_eq!(
                invoke(
                    &mut emu,
                    "advapi32.dll",
                    "AreAllAccessesGranted",
                    &[0x10, 0x20]
                ),
                0
            );
            assert_eq!(
                invoke(
                    &mut emu,
                    "advapi32.dll",
                    "AreAllAccessesGranted",
                    &[0x30, 0x20]
                ),
                1
            );
            emu.maps.write_string(data, "127.0.0.1");
            assert_eq!(
                invoke(&mut emu, "ws2_32.dll", "inet_addr", &[data]),
                0x0100_007f
            );
            emu.maps.write_wide_string(data, "folder");
            assert_eq!(
                invoke(&mut emu, "shlwapi.dll", "PathIsRelativeW", &[data]),
                1
            );
            assert_eq!(
                invoke(&mut emu, "dnsapi.dll", "DnsNameCompare_W", &[data, data]),
                1
            );
            let dc = invoke(&mut emu, "gdi32.dll", "CreateCompatibleDC", &[0]);
            assert_ne!(dc, 0);
            assert_eq!(invoke(&mut emu, "gdi32.dll", "GetDeviceCaps", &[dc, 14]), 1);
            assert_eq!(invoke(&mut emu, "gdi32.dll", "DeleteDC", &[dc]), 1);
            assert_eq!(invoke(&mut emu, "shell32.dll", "DllCanUnloadNow", &[]), 1);
            assert_eq!(
                invoke(&mut emu, "iphlpapi.dll", "GetIfTable", &[0, 0, 0]),
                ERROR_INVALID_PARAMETER
            );
            for (dll, open, close) in [
                ("wininet.dll", "InternetOpenW", "InternetCloseHandle"),
                ("winhttp.dll", "WinHttpOpen", "WinHttpCloseHandle"),
            ] {
                let session = invoke(&mut emu, dll, open, &[0, 0, 0, 0, 0]);
                assert_ne!(session, 0);
                assert_eq!(invoke(&mut emu, dll, close, &[session]), 1);
            }
            emu.set_api_mode(ApiMode::SimulateFail);
            assert_eq!(
                invoke(&mut emu, "urlmon.dll", "URLDownloadToFileW", &[0; 5]),
                E_NOTIMPL
            );
            emu.set_last_error(ERROR_ACCESS_DENIED);
            invoke(&mut emu, "comctl32.dll", "InitCommonControls", &[]);
            for (size, flags, expected) in [
                (8, ICC_WIN95_CLASSES, 1),
                (8, ICC_DATE_CLASSES | ICC_LINK_CLASS, 1),
                (7, ICC_WIN95_CLASSES, 0),
                (8, u32::MAX, 0),
            ] {
                assert!(emu.maps.write_dword(data, size));
                assert!(emu.maps.write_dword(data + 4, flags));
                assert_eq!(
                    invoke(&mut emu, "comctl32.dll", "InitCommonControlsEx", &[data]),
                    expected
                );
            }
            assert_eq!(
                invoke(&mut emu, "comctl32.dll", "InitCommonControlsEx", &[0]),
                0
            );
            for (name, result) in [
                ("IsAppThemed", 1),
                ("IsThemeActive", 1),
                ("GetThemeAppProperties", STAP_ALLOW_NONCLIENT as u64),
            ] {
                assert_eq!(invoke(&mut emu, "uxtheme.dll", name, &[]), result);
            }
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            let address = invoke(&mut emu, "gdiplus.dll", "GdipAlloc", &[16]);
            assert_ne!(address, 0);
            invoke(&mut emu, "gdiplus.dll", "GdipFree", &[address]);
        }
    }
}

#[test]
fn rectangle_geometry_uses_signed_coordinates_and_caller_buffers() {
    use helpers::call_winapi_export;
    for is64 in [false, true] {
        let (mut emu, first) = helpers::prepare_winapi(is64);
        let second = first + 32;
        let output = second + 32;
        let mut call =
            |name: &str, args: &[u64]| call_winapi_export(&mut emu, "user32.dll", name, args);
        assert_eq!(
            call("SetRect", &[first, (-6_i32) as u64, (-4_i32) as u64, 4, 6]),
            1
        );
        assert_eq!(call("IsRectEmpty", &[first]), 0);
        for (x, y, inside) in [
            (-6_i32, -4_i32, 1),
            (0, 5, 1),
            (4, 0, 0),
            (0, 6, 0),
            (-7, 0, 0),
        ] {
            let point = (u64::from(y as u32) << i32::BITS) | u64::from(x as u32);
            let args = if is64 {
                vec![first, point]
            } else {
                vec![first, x as u64, y as u64]
            };
            assert_eq!(call("PtInRect", &args), inside);
        }
        assert_eq!(call("CopyRect", &[output, first]), 1);
        assert_eq!(call("EqualRect", &[first, output]), 1);
        assert_eq!(call("OffsetRect", &[output, 2, 3]), 1);
        assert_eq!(call("InflateRect", &[output, 2, 3]), 1);
        assert_eq!(
            call(
                "SetRect",
                &[second, (-6_i32) as u64, (-4_i32) as u64, 8, 12]
            ),
            1
        );
        assert_eq!(call("EqualRect", &[second, output]), 1);
        assert_eq!(call("SetRect", &[second, 0, (-4_i32) as u64, 4, 6]), 1);
        assert_eq!(call("IntersectRect", &[output, first, second]), 1);
        assert_eq!(call("EqualRect", &[second, output]), 1);
        assert_eq!(call("UnionRect", &[output, first, second]), 1);
        assert_eq!(call("EqualRect", &[first, output]), 1);
        assert_eq!(call("SubtractRect", &[output, output, second]), 1);
        assert_eq!(
            call("SetRect", &[second, (-6_i32) as u64, (-4_i32) as u64, 0, 6]),
            1
        );
        assert_eq!(call("EqualRect", &[second, output]), 1);
        assert_eq!(call("SubtractRect", &[output, output, first]), 0);
        assert_eq!(call("SetRectEmpty", &[first]), 1);
        assert_eq!(call("EqualRect", &[first, output]), 1);
    }
}

#[test]
fn title_queries_use_the_window_and_respect_guest_capacity() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::ERROR_INVALID_WINDOW_HANDLE;

    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let hwnd = create_test_window(&mut emu, "Initial");
        let dll = "user32.dll";
        let buffer = data + 64;
        emu.maps.write_wide_string(data, "café");
        assert_eq!(call(&mut emu, dll, "SetWindowTextW", &[hwnd, data]), 1);
        assert!(emu.maps.write_dword(buffer, u32::MAX));
        // An x64 caller may leave live values in unused argument registers.
        let args = if is64 {
            vec![hwnd, buffer, 4]
        } else {
            vec![hwnd]
        };
        assert_eq!(call(&mut emu, dll, "GetWindowTextLengthA", &args), 4);
        assert_eq!(emu.maps.read_dword(buffer), Some(u32::MAX));
        assert_eq!(call(&mut emu, dll, "GetWindowTextA", &[hwnd, buffer, 3]), 2);
        assert_eq!(&*emu.maps.read_bytes(buffer, 4), b"ca\0\xff");
        assert_eq!(call(&mut emu, dll, "GetWindowTextW", &[hwnd, buffer, 5]), 4);
        assert_eq!(emu.maps.read_wide_string(buffer), "café");
        assert!(emu.guest_windows.destroy(hwnd));
        assert_eq!(call(&mut emu, dll, "GetWindowTextLengthW", &[hwnd]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_WINDOW_HANDLE);
    }
}

#[test]
fn shell_item_ids_traverse_and_clone_guest_records() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let lib = "shell32.dll";
        let list = [3, 0, 0x41, 4, 0, 0x42, 0x43, 0, 0];
        assert!(emu.maps.write_bytes(data, &list));
        assert_eq!(call(&mut emu, lib, "ILGetSize", &[data]), 9);
        assert_eq!(call(&mut emu, lib, "ILGetNext", &[data]), data + 3);
        assert_eq!(call(&mut emu, lib, "ILGetNext", &[data + 7]), 0);
        assert_eq!(call(&mut emu, lib, "ILFindLastID", &[data]), data + 3);
        let first = call(&mut emu, lib, "ILCloneFirst", &[data]);
        assert_ne!(first, 0);
        assert_eq!(call(&mut emu, lib, "ILFindChild", &[first, data]), data + 3);
        let copy = call(&mut emu, lib, "ILCombine", &[first, data + 3]);
        assert_ne!(copy, 0);
        assert_eq!(emu.maps.read_bytes(copy, list.len()).as_ref(), &list);
        assert_eq!(call(&mut emu, lib, "ILRemoveLastID", &[copy]), 1);
        assert_eq!(call(&mut emu, lib, "ILGetSize", &[copy]), 5);
        assert_eq!(call(&mut emu, lib, "ILGetSize", &[data]), 9);
        call(&mut emu, lib, "ILFree", &[first]);
        call(&mut emu, lib, "SHFree", &[copy]);
        assert!(emu.maps.write_word(data, 1));
        assert_eq!(call(&mut emu, lib, "ILGetSize", &[data]), 0);
    }
}

fn create_test_window(emu: &mut Emu, title: &str) -> u64 {
    emu.guest_windows
        .insert(Window {
            style: 0,
            extended_style: 0,
            instance: 0,
            user_data: 0,
            parent: 0,
            class: "STATIC".into(),
            title: title.into(),
            thread: emu.current_thread().id,
            process: 1,
            control_id: 0,
            enabled: true,
            wide: true,
            bounds: [0; 4],
            menu: 0,
        })
        .unwrap()
}

#[test]
fn oem_and_window_handles_keep_guest_data_isolated() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::controls::WS_CHILD;
    let lib = "user32.dll";
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let out = data + 64;
        emu.maps.write_wide_string(data, "é");
        emu.maps.write_dword(out, u32::MAX);
        assert_eq!(call(&mut emu, lib, "CharToOemW", &[data, out]), 1);
        assert_eq!(emu.maps.read_bytes(out, 3).as_ref(), &[0x82, 0, 0xff]);
        assert_eq!(call(&mut emu, lib, "OemToCharBuffW", &[out, data, 1]), 1);
        assert_eq!(emu.maps.read_word(data), Some('é' as u16));

        let parent = create_test_window(&mut emu, "parent");
        emu.maps.write_wide_string(data, "STATIC");
        let child = call(
            &mut emu,
            lib,
            "CreateWindowExW",
            &[0, data, 0, WS_CHILD.into(), 0, 0, 0, 0, parent, 17, 0, 0],
        );
        assert_ne!(child, 0);
        assert_eq!(call(&mut emu, lib, "GetParent", &[child]), parent);
        assert_eq!(call(&mut emu, lib, "SetParent", &[parent, child]), 0);
        assert_eq!(call(&mut emu, lib, "IsChild", &[parent, child]), 1);
        assert_eq!(call(&mut emu, lib, "GetDlgItem", &[parent, 17]), child);
        assert_eq!(call(&mut emu, lib, "EnableWindow", &[child, 0]), 0);
        assert_eq!(call(&mut emu, lib, "IsWindowEnabled", &[child]), 0);
        assert_eq!(call(&mut emu, lib, "EnableWindow", &[child, 1]), 1);
        emu.maps.write_qword(out, u64::MAX);
        call(&mut emu, lib, "GetWindowThreadProcessId", &[child, out]);
        assert_ne!(emu.maps.read_dword(out), Some(u32::MAX));
        assert_eq!(emu.maps.read_dword(out + 4), Some(u32::MAX));
        let (mut other, _) = prepare_winapi(is64);
        assert_eq!(call(&mut other, lib, "IsWindow", &[child]), 0);
        assert_eq!(call(&mut emu, lib, "DestroyWindow", &[parent]), 1);
        assert_eq!(call(&mut emu, lib, "IsWindow", &[child]), 0);
    }
}

#[test]
fn unsupported_shell_launch_does_not_report_a_started_process() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::policy::ApiMode;
    for is64 in [false, true] {
        let (mut emu, path) = prepare_winapi(is64);
        emu.maps.write_wide_string(path, "notepad.exe");
        emu.set_api_mode(ApiMode::SimulateFail);
        let result = call(
            &mut emu,
            "shell32.dll",
            "ShellExecuteW",
            &[0, 0, path, 0, 0, 1],
        );
        assert!(result <= 32);
    }
}

#[test]
fn shell_folder_queries_share_paths_and_respect_the_output_contract() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::read_pointer;
    use crate::windows::constants::{errors::*, shell::*};
    for is64 in [false, true] {
        let (mut emu, id) = prepare_winapi(is64);
        let (output, pointer) = (id + 32, id + 512);
        emu.maps
            .write_bytes(id, &FOLDER_ID_LOCAL_APP_DATA.to_le_bytes());
        let dll = "shell32.dll";
        for (index, root) in [(2, r"C:\"), (25, r"Z:\")] {
            emu.maps.write_wide_string(output, root);
            let expected = call(&mut emu, "kernel32.dll", "GetDriveTypeW", &[output]);
            assert_eq!(call(&mut emu, dll, "DriveType", &[index]), expected);
            assert_eq!(call(&mut emu, dll, "RealDriveType", &[index, 0]), expected);
        }

        assert_eq!(
            call(&mut emu, dll, "SHGetKnownFolderPath", &[id, 0, 0, pointer]),
            S_OK
        );
        let allocated = read_pointer(&emu, pointer).unwrap();
        let path = emu.maps.read_wide_string(allocated);
        assert!(path.ends_with(r"\AppData\Local"));
        emu.maps.write_word(output, 0xabcd);
        assert_eq!(
            call(&mut emu, dll, "SHGetFolderPathEx", &[id, 0, 0, output, 1]),
            HRESULT_INSUFFICIENT_BUFFER
        );
        assert_eq!(emu.maps.read_word(output), Some(0xabcd));
        assert_eq!(
            call(&mut emu, dll, "SHGetFolderPathEx", &[id, 0, 0, output, 128]),
            S_OK
        );
        assert_eq!(emu.maps.read_wide_string(output), path);
        call(&mut emu, "ole32.dll", "CoTaskMemFree", &[allocated]);
    }
}

#[test]
fn shell_argument_array_uses_guest_pointers_and_local_free() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::{pointer_size, read_pointer};
    for is64 in [false, true] {
        let (mut emu, line) = prepare_winapi(is64);
        let count = line + 512;
        emu.maps
            .write_wide_string(line, r#""a b.exe" "two words" "" tail"#);
        let array = call(
            &mut emu,
            "shell32.dll",
            "CommandLineToArgvW",
            &[line, count],
        );
        assert_ne!(array, 0);
        assert_eq!(emu.maps.read_dword(count), Some(4));
        let width = pointer_size(&emu);
        for (index, expected) in ["a b.exe", "two words", "", "tail"].iter().enumerate() {
            let pointer = read_pointer(&emu, array + index as u64 * width).unwrap();
            assert_eq!(&emu.maps.read_wide_string(pointer), expected);
        }
        assert_eq!(read_pointer(&emu, array + 4 * width), Some(0));
        assert_eq!(call(&mut emu, "kernel32.dll", "LocalFree", &[array]), 0);
    }
}

#[test]
fn display_enumeration_calls_back_with_the_primary_monitor_and_clip() {
    use crate::tests::helpers::{call_winapi_export, prepare_winapi};
    use crate::winapi::policy::ApiMode;

    const CALLBACK: u64 = 0x2800_0000;
    const SCREEN: [i32; 4] = [0, 0, 1920, 1080];
    for is64 in [true, false] {
        let (mut emu, data) = prepare_winapi(is64);
        emu.set_api_mode(ApiMode::SimulateSuccess);
        emu.maps
            .create_map(
                "monitor_callback",
                CALLBACK,
                0x1000,
                Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        // MONITORENUMPROC returns with RET on x64 and RET 16 on x86.
        let ret: &[u8] = if is64 { &[0xc3] } else { &[0xc2, 16, 0] };
        emu.maps.write_bytes(CALLBACK, ret);
        emu.set_pc(CALLBACK + 0x80);
        let point_args: &[u64] = if is64 { &[0, 0] } else { &[0, 0, 0] };
        let monitor = call_winapi_export(&mut emu, "user32.dll", "MonitorFromPoint", point_args);
        assert_ne!(monitor, 0);

        for (clip, expected, more) in [
            (None, Some(SCREEN), true),
            (Some([-5, 2, 9, 7]), Some([0, 2, 9, 7]), true),
            (Some([-5, 0, 0, 7]), None, true),
            (None, Some(SCREEN), false),
        ] {
            let clip_ptr = clip.map_or(0, |edges: [i32; 4]| {
                let addr = data + 16;
                assert!(
                    emu.maps
                        .write_bytes(addr, edges.map(i32::to_le_bytes).as_flattened())
                );
                addr
            });
            emu.maps.write_dword(data, 0);
            emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
                if pc == CALLBACK {
                    let [handle, dc, rect, context] = if is64 {
                        let r = emu.regs();
                        [r.rcx, r.rdx, r.r8, r.r9]
                    } else {
                        [4, 8, 12, 16].map(|offset| {
                            emu.maps
                                .read_dword(emu.sp() + offset)
                                .map(u64::from)
                                .unwrap()
                        })
                    };
                    assert_eq!((handle, dc, context), (monitor, 0, data));
                    let edges = [0, 4, 8, 12]
                        .map(|offset| emu.maps.read_dword(rect + offset).unwrap() as i32);
                    assert_eq!(Some(edges), expected);
                    let count = emu.maps.read_dword(context).unwrap();
                    emu.maps.write_dword(context, count + 1);
                    emu.regs_mut().rax = u64::from(more);
                }
                true
            });
            let result = call_winapi_export(
                &mut emu,
                "user32.dll",
                "EnumDisplayMonitors",
                &[0, clip_ptr, CALLBACK, data],
            );
            assert_eq!(
                emu.maps.read_dword(data),
                Some(u32::from(expected.is_some())),
                "callback count"
            );
            assert_eq!(result != 0, more);
        }
    }
}

#[test]
fn gdi_objects_keep_their_type_selection_guest_layout_and_display_profile() {
    use crate::tests::helpers::{call_winapi_export, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let call =
            |emu: &mut emu::Emu, api, args: &[u64]| call_winapi_export(emu, "gdi32.dll", api, args);
        let dc = call(&mut emu, "CreateCompatibleDC", &[0]);
        for (index, value) in [(2, 1), (8, 1920), (10, 1080), (12, 32), (14, 1), (88, 96)] {
            assert_eq!(call(&mut emu, "GetDeviceCaps", &[dc, index]), value);
        }
        assert_eq!(call(&mut emu, "GetDeviceCaps", &[0, 8]), 0);
        let bitmap = call(&mut emu, "CreateCompatibleBitmap", &[dc, 37, 9]);
        let saved = call(&mut emu, "GetCurrentObject", &[dc, 7]);
        assert_eq!(call(&mut emu, "SelectObject", &[dc, bitmap]), saved);
        assert_eq!(call(&mut emu, "GetObjectType", &[bitmap]), 7);
        assert_eq!(call(&mut emu, "GetObjectType", &[dc]), 10);
        let size = if is64 { 32 } else { 24 };
        assert_eq!(call(&mut emu, "GetObjectW", &[bitmap, 0, 0]), size);
        assert_eq!(call(&mut emu, "GetObjectA", &[bitmap, size - 1, data]), 0);
        assert_eq!(call(&mut emu, "GetObjectA", &[bitmap, size, data]), size);
        assert_eq!(emu.maps.read_dword(data + 4), Some(37));
        assert_eq!(emu.maps.read_dword(data + 12), Some(6));
        assert_eq!(emu.maps.read_word(data + 18), Some(1));
        let brush = call(&mut emu, "CreateSolidBrush", &[0x123456]);
        assert_eq!(
            call(&mut emu, "GetObjectW", &[brush, 32, data]),
            if is64 { 16 } else { 12 }
        );
        assert_eq!(emu.maps.read_dword(data + 4), Some(0x123456));
        assert_eq!(call(&mut emu, "DeleteObject", &[dc]), 0);
        assert_eq!(call(&mut emu, "SelectObject", &[dc, saved]), bitmap);
        assert_eq!(call(&mut emu, "DeleteObject", &[bitmap]), 1);
        assert_eq!(call(&mut emu, "GetObjectType", &[bitmap]), 0);
        assert_eq!(call(&mut emu, "SaveDC", &[dc]), 1);
        assert_eq!(call(&mut emu, "SetDCBrushColor", &[dc, 0x123456]), 0xffffff);
        assert_eq!(call(&mut emu, "GetDCBrushColor", &[dc]), 0x123456);
        assert_eq!(call(&mut emu, "SetViewportOrgEx", &[dc, 7, 9, 0]), 1);
        assert_eq!(call(&mut emu, "SetViewportOrgEx", &[dc, 2, 3, data]), 1);
        assert_eq!(emu.maps.read_dword(data), Some(7));
        assert_eq!(emu.maps.read_dword(data + 4), Some(9));
        assert_eq!(call(&mut emu, "RestoreDC", &[dc, 1]), 1);
        assert_eq!(call(&mut emu, "GetDCBrushColor", &[dc]), 0xffffff);
    }
}

#[test]
fn queued_messages_use_the_guest_msg_layout_and_only_quit_returns_zero() {
    use crate::serialization::Serialization;
    use crate::winapi::policy::{ApiDisposition, ApiMode};
    use crate::windows::constants::errors::ERROR_CALL_NOT_IMPLEMENTED;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let dll = "user32.dll";
        let size = if is64 { 48 } else { 32 };
        assert!(emu.maps.write_bytes(data, &[0xa5; 56]));
        assert_eq!(call(&mut emu, dll, "PeekMessageA", &[data, 0, 0, 0, 0]), 0);
        assert_eq!(emu.maps.read_byte(data), Some(0xa5));
        for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
            emu.set_api_mode(mode);
            assert_eq!(
                call(&mut emu, dll, "GetMessageW", &[data, 0, 0, 0]),
                u64::from(u32::MAX)
            );
            let outcome = emu
                .api_policy
                .last_outcome
                .as_ref()
                .expect("policy outcome");
            assert_eq!(outcome.disposition, ApiDisposition::SimulatedFailure);
            assert_eq!(outcome.result, Some(u64::from(u32::MAX)));
            assert_eq!(outcome.last_error, Some(ERROR_CALL_NOT_IMPLEMENTED));
            assert_eq!(emu.maps.read_byte(data), Some(0xa5));
        }
        let hwnd = create_test_window(&mut emu, "test");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PostMessageW",
                &[hwnd, WM_APP as u64, 123, 456]
            ),
            1
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PeekMessageW",
                &[data, hwnd, WM_APP as u64, WM_APP as u64, 0]
            ),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "GetMessageA",
                &[data, hwnd, WM_APP as u64, WM_APP as u64]
            ),
            1
        );
        let (message, wparam, lparam) = if is64 { (8, 16, 24) } else { (4, 8, 12) };
        assert_eq!(emu.maps.read_dword(data + message), Some(WM_APP));
        assert_eq!(emu.maps.read_dword(data + wparam), Some(123));
        assert_eq!(emu.maps.read_dword(data + lparam), Some(456));
        assert_eq!(emu.maps.read_byte(data + size), Some(0xa5));
        call(&mut emu, dll, "PostQuitMessage", &[42]);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "GetMessageW",
                &[data, 0, WM_APP as u64, WM_APP as u64]
            ),
            0
        );
        assert_eq!(emu.maps.read_dword(data + message), Some(WM_QUIT));
        assert_eq!(emu.maps.read_dword(data + wparam), Some(42));
    }
}

#[test]
fn shared_window_geometry_resources_and_station_lifetimes() {
    use helpers::call_winapi_export as call;
    const UOI_NAME: u64 = 2;
    const IDC_ARROW: u64 = 32512;
    const SWP_NOZORDER: u64 = 4;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let dll = "user32.dll";
        let hwnd = create_test_window(&mut emu, "sample");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "SetWindowPos",
                &[hwnd, 0, 10, 20, 150, 75, SWP_NOZORDER]
            ),
            1
        );
        assert_eq!(call(&mut emu, dll, "GetWindowRect", &[hwnd, data]), 1);
        assert_eq!(emu.maps.read_dword(data), Some(10));
        assert_eq!(emu.maps.read_dword(data + 12), Some(95));
        assert_eq!(call(&mut emu, dll, "GetClientRect", &[hwnd, data]), 1);
        assert_eq!(emu.maps.read_dword(data), Some(0));
        assert_eq!(emu.maps.read_dword(data + 12), Some(75));
        assert_eq!(call(&mut emu, dll, "GetClientRect", &[0, data]), 0);
        let station = call(&mut emu, dll, "GetProcessWindowStation", &[]);
        for (name, unit_bytes) in [
            ("GetUserObjectInformationA", 1),
            ("GetUserObjectInformationW", 2),
        ] {
            let capacity = ("WinSta0\0".len() * unit_bytes) as u32;
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    name,
                    &[station, UOI_NAME, data, 1, data + 32]
                ),
                0
            );
            assert_eq!(emu.maps.read_dword(data + 32), Some(capacity));
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    name,
                    &[station, UOI_NAME, data, u64::from(capacity), data + 32]
                ),
                1
            );
        }
        let cursor = call(&mut emu, dll, "LoadCursorA", &[0, IDC_ARROW]);
        assert_ne!(cursor, 0);
        assert_eq!(call(&mut emu, dll, "LoadCursorW", &[0, IDC_ARROW]), cursor);
        assert_eq!(call(&mut emu, dll, "SetCursor", &[cursor]), 0);
        assert_eq!(call(&mut emu, dll, "GetCursor", &[]), cursor);
        assert_eq!(call(&mut emu, dll, "LoadCursorW", &[0, 7]), 0);
        assert_eq!(call(&mut emu, dll, "SetCursorPos", &[123, 456]), 1);
        assert_eq!(call(&mut emu, dll, "GetCursorPos", &[data]), 1);
        assert_eq!(emu.maps.read_dword(data + 4), Some(456));
        let menu = call(&mut emu, dll, "CreateMenu", &[]);
        assert_ne!(menu, 0);
        assert_eq!(call(&mut emu, dll, "GetMenuItemCount", &[menu]), 0);
        assert_eq!(call(&mut emu, dll, "DeleteMenu", &[menu, 0, 0]), 0);
        assert_eq!(call(&mut emu, dll, "DestroyMenu", &[menu]), 1);
        assert_eq!(call(&mut emu, dll, "DestroyMenu", &[menu]), 0);
    }
}

#[test]
fn shell_runtime_exports_follow_their_hresult_and_output_contracts() {
    use crate::tests::helpers::{call_winapi_export, prepare_winapi};
    use crate::winapi::common::memory::{read_pointer, write_pointer};
    use crate::windows::constants::errors::{HRESULT_E_INVALID_ARG, REGDB_E_CLASSNOTREG, S_FALSE};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        emu.set_last_error(ERROR_ACCESS_DENIED);
        assert_eq!(
            call_winapi_export(&mut emu, "shell32.dll", "DllCanUnloadNow", &[]),
            S_FALSE
        );
        for (class_id, status) in [(0, HRESULT_E_INVALID_ARG), (data + 32, REGDB_E_CLASSNOTREG)] {
            write_pointer(&mut emu, data, u32::MAX.into());
            assert_eq!(
                call_winapi_export(
                    &mut emu,
                    "shell32.dll",
                    "DllGetActivationFactory",
                    &[class_id, data]
                ),
                status
            );
            assert_eq!(read_pointer(&emu, data), Some(0));
        }
        assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
    }
}

#[test]
fn user_timers_deliver_coalesced_messages_and_dispatch_live_guest_callbacks() {
    use crate::serialization::Serialization;
    use helpers::{call_winapi_export as call, prepare_winapi};

    const DLL: &str = "user32.dll";
    const CALLBACK: u64 = 0x2800_0000;
    let timer = u64::from(WM_TIMER);
    for is64 in [false, true] {
        let (mut emu, msg) = prepare_winapi(is64);
        let word = if is64 { 8 } else { 4 };
        let hwnd = create_test_window(&mut emu, "timer");
        emu.maps
            .create_map(
                "timer_callback",
                CALLBACK,
                0x1000,
                Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        let ret: &[u8] = if is64 { &[0xc3] } else { &[0xc2, 16, 0] };
        emu.maps.write_bytes(CALLBACK, ret);
        emu.set_pc(CALLBACK + 0x80);
        let id = call(&mut emu, DLL, "SetTimer", &[0, 0xbeef, 1, CALLBACK]);
        assert_ne!(id, 0);
        assert_ne!(id, 0xbeef);
        assert_eq!(call(&mut emu, DLL, "PeekMessageW", &[msg, 0, 0, 0, 0]), 0);
        assert_ne!(call(&mut emu, DLL, "SetTimer", &[hwnd, 0, 100, 0]), 0);
        emu.tick += USER_TIMER_MINIMUM as usize;
        call(&mut emu, DLL, "PostMessageW", &[hwnd, WM_APP as u64, 0, 0]);
        assert_eq!(call(&mut emu, DLL, "GetMessageA", &[msg, 0, 0, 0]), 1);
        assert_eq!(emu.maps.read_dword(msg + word), Some(WM_APP));
        assert_eq!(
            call(&mut emu, DLL, "PeekMessageW", &[msg, 0, timer, timer, 0]),
            1
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        let generated_at = emu.tick as u32;
        assert_eq!(
            call(&mut emu, DLL, "GetMessageA", &[msg, u64::MAX, timer, timer]),
            1
        );
        assert_eq!(emu.maps.read_dword(msg + 2 * word), Some(id as u32));
        assert_eq!(emu.maps.read_dword(msg + 4 * word), Some(generated_at));
        emu.tick += 125;
        let callback_time = emu.tick as u32;
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if pc == CALLBACK {
                let args = if is64 {
                    let r = emu.regs();
                    [r.rcx, r.rdx, r.r8, r.r9]
                } else {
                    [4, 8, 12, 16]
                        .map(|offset| u64::from(emu.maps.read_dword(emu.sp() + offset).unwrap()))
                };
                assert_eq!(args, [0, timer, id, u64::from(callback_time)]);
                emu.regs_mut().rax = 0x1234_5678;
            }
            true
        });
        assert_eq!(call(&mut emu, DLL, "DispatchMessageW", &[msg]), 0x1234_5678);
        // A timer that expired repeatedly produces one message when the queue is inspected.
        assert_eq!(
            call(
                &mut emu,
                DLL,
                "PeekMessageA",
                &[msg, u64::MAX, timer, timer, 1]
            ),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                DLL,
                "PeekMessageA",
                &[msg, u64::MAX, timer, timer, 1]
            ),
            0
        );
        assert_eq!(call(&mut emu, DLL, "KillTimer", &[0, id]), 1);
        // The old MSG still exists. Its callback runs only while registered somewhere in the process.
        assert_eq!(call(&mut emu, DLL, "DispatchMessageA", &[msg]), 0);
        let mut peer = emu.current_thread().clone();
        peer.id += 1;
        emu.threads.push(peer);
        emu.current_thread_id = 1;
        let other = call(&mut emu, DLL, "SetTimer", &[0, 0, 500, CALLBACK]);
        emu.current_thread_id = 0;
        assert_ne!(other, id);
        assert_eq!(call(&mut emu, DLL, "DispatchMessageA", &[msg]), 0x1234_5678);
        assert_eq!(call(&mut emu, DLL, "KillTimer", &[0, other]), 1);
        assert_eq!(
            call(&mut emu, DLL, "GetMessageW", &[msg, hwnd, timer, timer]),
            1
        );
        assert_eq!(emu.maps.read_dword(msg + 2 * word), Some(0));
        assert_eq!(call(&mut emu, DLL, "DispatchMessageW", &[msg]), 0);
        assert_eq!(call(&mut emu, DLL, "KillTimer", &[hwnd, 0]), 1);
        emu.set_last_error(123);
        assert_eq!(call(&mut emu, DLL, "KillTimer", &[hwnd, 0]), 0);
        assert_eq!(emu.get_last_error(), 123);
    }
}

#[test]
fn timer_replacement_resets_the_deadline_and_leaves_materialized_messages_in_the_queue() {
    use crate::windows::constants::errors::ERROR_INVALID_WINDOW_HANDLE;
    use helpers::{call_winapi_export as call, prepare_winapi};
    const DLL: &str = "user32.dll";
    let timer = u64::from(WM_TIMER);
    for is64 in [false, true] {
        let (mut emu, msg) = prepare_winapi(is64);
        let word = if is64 { 8 } else { 4 };
        let hwnd = create_test_window(&mut emu, "owner");
        let id = call(&mut emu, DLL, "SetTimer", &[0, 0, 100, 0]);
        emu.tick += 90;
        let start = emu.tick;
        let mut peer = emu.current_thread().clone();
        peer.id += 1;
        emu.threads.push(peer);
        emu.current_thread_id = 1;
        assert_eq!(call(&mut emu, DLL, "SetTimer", &[0, id, 50, 0]), id);
        emu.tick += 10;
        assert_eq!(
            call(&mut emu, DLL, "PeekMessageA", &[msg, 0, timer, timer, 0]),
            0
        );
        assert_eq!(
            call(&mut emu, DLL, "GetMessageW", &[msg, 0, timer, timer]),
            1
        );
        assert_eq!(emu.tick, start + 50);
        emu.tick += 50;
        assert_eq!(
            call(&mut emu, DLL, "PeekMessageA", &[msg, 0, timer, timer, 0]),
            1
        );
        // Replacing a windowless timer transfers delivery to this thread. The old thread can still cancel its ID.
        emu.current_thread_id = 0;
        assert_eq!(
            call(&mut emu, DLL, "PeekMessageA", &[msg, 0, timer, timer, 1]),
            0
        );
        assert_eq!(call(&mut emu, DLL, "KillTimer", &[0, id]), 1);
        emu.current_thread_id = 1;
        assert_eq!(
            call(&mut emu, DLL, "GetMessageW", &[msg, 0, timer, timer]),
            1
        );
        assert_eq!(emu.maps.read_dword(msg + 2 * word), Some(id as u32));
        emu.current_thread_id = 0;
        let mut child = emu.guest_windows.get(hwnd).unwrap().clone();
        child.parent = hwnd;
        let child = emu.guest_windows.insert(child).unwrap();
        for window in [hwnd, child] {
            assert_ne!(call(&mut emu, DLL, "SetTimer", &[window, 33, 10, 0]), 0);
        }
        assert_eq!(call(&mut emu, DLL, "DestroyWindow", &[hwnd]), 1);
        emu.tick += 10;
        assert_eq!(
            call(&mut emu, DLL, "PeekMessageW", &[msg, 0, timer, timer, 1]),
            0
        );
        assert_eq!(call(&mut emu, DLL, "SetTimer", &[hwnd, 33, 10, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_WINDOW_HANDLE);
    }
}
