use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;
use crate::windows::constants::system::FORMAT_MESSAGE_FROM_SYSTEM;
use itertools::iproduct;

use crate::maps::mem64::Permission;
use crate::tests::helpers::{self, call_winapi_export as call, prepare_winapi as prepare};
use crate::winapi::{winapi32, winapi64};
use crate::{emu32, emu64};

const MARKER: u32 = 0xcccc_cccc;

#[test]
fn character_navigation_returns_pointers_and_respects_boundaries() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["user32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let (next, prev, step) = if wide {
            emu.maps.write_wide_string(data, "AB");
            ("CharNextW", "CharPrevW", 2)
        } else {
            emu.maps.write_string(data, "AB");
            ("CharNextA", "CharPrevA", 1)
        };
        emu.set_last_error(MARKER as u64);
        for offset in 0..=2 {
            assert_eq!(
                call(&mut emu, library, next, &[data + offset * step]),
                data + (offset + 1).min(2) * step
            );
            assert_eq!(
                call(&mut emu, library, prev, &[data, data + offset * step]),
                data + offset.saturating_sub(1) * step
            );
        }
        assert_eq!(emu.get_last_error(), MARKER as u64);
    }
}

#[test]
fn extended_navigation_uses_dbcs_boundaries_and_the_requested_code_page() {
    for (is64, library) in iproduct!([false, true], ["user32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = prepare(is64);
        // CP932: both bytes in the first character are also valid lead bytes.
        assert!(emu.maps.write_bytes(data, &[0x81, 0x81, b'A', 0x81, 0]));
        for (code_page, next, previous) in [
            (CP_SHIFT_JIS as u64, 2, 0),
            (CP_WINDOWS_1252 as u64, 1, 1),
            (CP_ACP as u64, 1, 1),
        ] {
            assert_eq!(
                call(&mut emu, library, "CharNextExA", &[code_page, data, 0]),
                data + next
            );
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "CharPrevExA",
                    &[code_page, data, data + 2, 0]
                ),
                data + previous
            );
        }
        assert_eq!(
            call(
                &mut emu,
                library,
                "CharNextExA",
                &[CP_SHIFT_JIS as u64, data + 3, 0]
            ),
            data + 4
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "CharNextExA",
                &[CP_SHIFT_JIS as u64, data + 4, 0]
            ),
            data + 4
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "CharPrevExA",
                &[CP_SHIFT_JIS as u64, data, data, 0]
            ),
            data
        );
    }
}

#[test]
fn lead_byte_checks_agree_with_code_pages() {
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, _) = prepare(is64);
        assert_eq!(call(&mut emu, library, "IsDBCSLeadByte", &[0x81]), 0); // ACP is 1252.
        for (cp, byte, expected) in [
            (CP_WINDOWS_1252 as u64, 0x81, 0),
            (CP_SHIFT_JIS as u64, 0x81, 1),
            (CP_SHIFT_JIS as u64, 0xa0, 0),
            (CP_GBK as u64, 0xa0, 1),
            (CP_KOREAN as u64, 0xfe, 1),
            (CP_BIG5 as u64, 0x80, 0),
            (CP_JOHAB as u64, 0x84, 1),
        ] {
            assert_eq!(
                call(&mut emu, library, "IsDBCSLeadByteEx", &[cp, byte]),
                expected
            );
        }
    }
}

#[test]
fn rtl_init_unicode_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let ustr_addr = 0x100000u64;
    let src_addr = 0x100100u64;
    emu.maps
        .create_map("ustr_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"Hello" (wide) — 5 chars + null = 12 bytes
    let hello: [u8; 12] = [b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, 0, 0];
    for (i, &b) in hello.iter().enumerate() {
        emu.maps.write_byte(src_addr + i as u64, b);
    }

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[ustr_addr, src_addr],
    );

    let length = emu.maps.read_word(ustr_addr).unwrap();
    let max_length = emu.maps.read_word(ustr_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(ustr_addr + 8).unwrap();
    assert_eq!(length, 10, "Length must be 5 chars * 2 bytes = 10");
    assert_eq!(max_length, 12, "MaximumLength must be Length + 2");
    assert_eq!(buffer, src_addr, "Buffer must point to the source string");
}

#[test]
fn rtl_init_ansi_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let astr_addr = 0x100000u64;
    let src_addr = 0x100100u64;
    emu.maps
        .create_map("astr_io", 0x100000, 0x1000, Permission::READ_WRITE);

    emu.maps.write_string(src_addr, "Test");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitAnsiString,
        &[astr_addr, src_addr],
    );

    let length = emu.maps.read_word(astr_addr).unwrap();
    let max_length = emu.maps.read_word(astr_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(astr_addr + 8).unwrap();
    assert_eq!(length, 4, "Length must be 4 (strlen)");
    assert_eq!(max_length, 5, "MaximumLength must be Length + 1");
    assert_eq!(buffer, src_addr, "Buffer must point to the source string");
}

#[test]
fn rtl_init_unicode_string() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    let source_addr = 0x100100u64;
    emu.maps
        .create_map("unistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(source_addr, "Hello");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[dest_addr, source_addr],
    );

    // "Hello" = 5 UTF-16 code units = 10 bytes
    let length = emu.maps.read_word(dest_addr).unwrap();
    let max_length = emu.maps.read_word(dest_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(dest_addr + 8).unwrap();

    assert_eq!(length, 10, "UNICODE_STRING.Length should be 10 for 'Hello'");
    assert_eq!(max_length, 12, "UNICODE_STRING.MaximumLength should be 12");
    assert_eq!(
        buffer, source_addr,
        "UNICODE_STRING.Buffer should point to source"
    );
}

#[test]
fn rtl_init_unicode_string_null() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    emu.maps
        .create_map("unistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(dest_addr, 0xFFFFFFFF_FFFFFFFF);
    emu.maps.write_qword(dest_addr + 8, 0xFFFFFFFF_FFFFFFFF);

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[dest_addr, 0],
    );

    assert_eq!(emu.maps.read_qword(dest_addr).unwrap(), 0);
    assert_eq!(emu.maps.read_qword(dest_addr + 8).unwrap(), 0);
}

#[test]
fn rtl_init_ansi_string() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    let source_addr = 0x100100u64;
    emu.maps
        .create_map("ansistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(source_addr, "TestStr");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitAnsiString,
        &[dest_addr, source_addr],
    );

    let length = emu.maps.read_word(dest_addr).unwrap();
    let max_length = emu.maps.read_word(dest_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(dest_addr + 8).unwrap();

    assert_eq!(length, 7, "ANSI_STRING.Length should be 7 for 'TestStr'");
    assert_eq!(max_length, 8, "ANSI_STRING.MaximumLength should be 8");
    assert_eq!(
        buffer, source_addr,
        "ANSI_STRING.Buffer should point to source"
    );
}

#[test]
fn format_message_a() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::FormatMessageA,
        &[FORMAT_MESSAGE_FROM_SYSTEM as u64, 0, 0, 0],
    );
    assert_eq!(ret, 0, "FormatMessageA stub should return 0");
}

#[test]
fn lstrcmpiw_32() {
    helpers::setup();
    let mut emu = emu32();
    let a = 0x40000u64;
    let b = 0x40100u64;
    emu.maps
        .create_map("s", 0x40000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(a, "HELLO");
    emu.maps.write_wide_string(b, "hello");

    let eq = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::lstrcmpiW,
        &[a as u32, b as u32],
    ) as i32;
    assert_eq!(
        eq, 0,
        "case-insensitive compare of HELLO/hello should be equal"
    );

    emu.maps.write_wide_string(b, "world");
    let ne = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::lstrcmpiW,
        &[a as u32, b as u32],
    ) as i32;
    assert!(ne < 0, "'hello' should sort before 'world'");
}

#[test]
fn get_locale_info_a_32() {
    helpers::setup();
    let mut emu = emu32();
    let buf = 0x40000u64;
    emu.maps
        .create_map("lc", buf, 0x1000, Permission::READ_WRITE);
    // Keep the query ID independent of the emulator table for this SDK regression.
    const CALENDAR_TYPE_LCTYPE: u64 = 0x1009;
    for (lctype, expected) in [(LOCALE_SMONTHNAME1, "January"), (CALENDAR_TYPE_LCTYPE, "1")] {
        let n = helpers::call_winapi32(
            &mut emu,
            winapi32::kernel32::GetLocaleInfoA,
            &[LOCALE_USER_DEFAULT as u32, lctype as u32, buf as u32, 0x100],
        );
        assert_eq!(n, expected.len() as u32 + 1, "GetLocaleInfoA wrong length");
        assert_eq!(emu.maps.read_string(buf), expected);
    }
}

#[test]
fn character_classification_uses_scalar_arguments_in_both_encodings() {
    let cases = [
        ('A', [true, true, false, true]),
        ('z', [true, true, true, false]),
        ('5', [false, true, false, false]),
        ('!', [false, false, false, false]),
        ('\0', [false, false, false, false]),
        ('é', [true, true, true, false]),
        ('É', [true, true, false, true]),
    ];
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["user32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, _) = prepare(is64);
        for (character, expected) in cases {
            for (api, expected) in [
                "IsCharAlpha",
                "IsCharAlphaNumeric",
                "IsCharLower",
                "IsCharUpper",
            ]
            .into_iter()
            .zip(expected)
            {
                let name = format!("{api}{}", if wide { "W" } else { "A" });
                emu.regs_mut().rax = MARKER as u64;
                emu.set_last_error(MARKER as u64);
                assert_eq!(
                    call(&mut emu, library, &name, &[character as u64]),
                    expected as u64,
                    "{library}!{name}({character:?}), x64={is64}"
                );
                assert_eq!(emu.get_last_error(), MARKER as u64);
            }
        }
        // High argument bits are outside CHAR/WCHAR, not a guest pointer.
        assert_eq!(
            call(
                &mut emu,
                library,
                if wide {
                    "IsCharAlphaNumericW"
                } else {
                    "IsCharAlphaNumericA"
                },
                &[0x1234_0000 | b'!' as u64]
            ),
            0
        );
        if wide {
            assert_eq!(call(&mut emu, library, "IsCharAlphaW", &['Ж' as u64]), 1);
            assert_eq!(call(&mut emu, library, "IsCharAlphaW", &[0xd800]), 0);
        } else {
            assert_eq!(call(&mut emu, library, "IsCharAlphaA", &[0x8c]), 1); // CP1252 Œ.
        }
    }
}

#[test]
fn computer_name_reports_required_capacity_and_utf16_length() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        emu.cfg.host_name = if wide { "NODE😀" } else { "HOSTNAME-LONG" }.to_string();
        let mut expected: Vec<u8> = if wide {
            emu.cfg
                .host_name
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect()
        } else {
            emu.cfg.host_name.as_bytes().to_vec()
        };
        let unit_size = if wide { 2 } else { 1 };
        let length = expected.len() / unit_size;
        expected.resize(expected.len() + unit_size, 0);
        let size = data + 0x100;
        let api = if wide {
            "GetComputerNameW"
        } else {
            "GetComputerNameA"
        };
        for capacity in [0, 1, length, length + 1, length + 2] {
            emu.maps.memset(data, 0xcc, expected.len() + 4);
            emu.maps.write_dword(size, capacity as u32);
            emu.maps.write_dword(size + 4, MARKER);
            emu.set_last_error(MARKER as u64);
            let result = call(&mut emu, library, api, &[data, size]);
            if capacity <= length {
                assert_eq!(result, 0, "{api} capacity {capacity}");
                assert_eq!(emu.get_last_error(), ERROR_BUFFER_OVERFLOW);
                assert_eq!(emu.maps.read_dword(size), Some((length + 1) as u32));
                assert_eq!(
                    emu.maps.read_bytes(data, expected.len()).as_ref(),
                    vec![0xcc; expected.len()]
                );
            } else {
                assert_eq!(result, 1);
                assert_eq!(emu.maps.read_dword(size), Some(length as u32));
                assert_eq!(emu.maps.read_bytes(data, expected.len()).as_ref(), expected);
                assert_eq!(emu.get_last_error(), MARKER as u64);
            }
            assert_eq!(
                emu.maps.read_dword(data + expected.len() as u64),
                Some(MARKER)
            );
            assert_eq!(emu.maps.read_dword(size + 4), Some(MARKER));
        }
    }
}

#[test]
fn computer_name_rejects_unwritable_outputs_without_partial_writes() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let size = data + 0x100;
        let short = data + 0x10000;
        emu.maps
            .create_map("short_output", short, 1, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_byte(short, 0xcc);
        emu.maps.write_dword(size, 64);
        let api = if wide {
            "GetComputerNameW"
        } else {
            "GetComputerNameA"
        };
        assert_eq!(call(&mut emu, library, api, &[short, size]), 0);
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        assert_eq!(emu.maps.read_byte(short), Some(0xcc));
        assert_eq!(emu.maps.read_dword(size), Some(64));
        emu.maps.memset(data, 0xcc, 64);
        assert_eq!(call(&mut emu, library, api, &[data, 0]), 0);
        assert_eq!(emu.maps.read_dword(data), Some(MARKER));
    }
}
