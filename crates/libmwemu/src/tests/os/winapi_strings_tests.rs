use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;
use crate::windows::constants::system::{
    FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_STRING, FORMAT_MESSAGE_FROM_SYSTEM,
};
use itertools::iproduct;

use crate::emu::Emu;
use crate::emu32;
use crate::kernel::windows::rtl;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{self, call_winapi_export as call, prepare_winapi as prepare};
use crate::winapi::winapi32;

const MARKER: u32 = 0xcccc_cccc;

#[test]
fn locale_info_writes_only_to_the_output_buffer() {
    for is64 in [false, true] {
        let (mut emu, output) = prepare(is64);
        const GUARD: u64 = 0x2600_0000;
        emu.maps
            .create_map(
                "scalar_guard",
                GUARD,
                size_of::<u32>() as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        emu.maps.write_dword(GUARD, MARKER);
        // Both scalar arguments can happen to contain a mapped address.
        for (kind, capacity) in [(GUARD, 32), (LOCALE_SMONTHNAME1, GUARD)] {
            let result = call(
                &mut emu,
                "kernel32.dll",
                "GetLocaleInfoW",
                &[LOCALE_USER_DEFAULT, kind, output, capacity],
            );
            assert_eq!(emu.maps.read_dword(GUARD), Some(MARKER));
            if kind == LOCALE_SMONTHNAME1 {
                assert_eq!(result, "January".len() as u64 + 1);
                assert_eq!(emu.maps.read_wide_string(output), "January");
            } else {
                assert_eq!(result, 0);
                assert_eq!(emu.get_last_error(), ERROR_INVALID_FLAGS);
            }
        }
    }
}

#[test]
fn code_page_queries_agree_across_architectures_and_library_routes() {
    for is64 in [false, true] {
        let (mut emu, _) = prepare(is64);
        for (library, (name, expected)) in iproduct!(
            ["kernel32.dll", "kernelbase.dll"],
            [
                ("GetACP", CP_WINDOWS_1252),
                ("GetOEMCP", CP_IBM437),
                ("GetUserDefaultLCID", EN_US_LOCALE)
            ]
        ) {
            emu.set_last_error(MARKER as u64);
            assert_eq!(
                call(&mut emu, library, name, &[]),
                expected as u64,
                "{library}!{name}, is64={is64}"
            );
            assert_eq!(emu.get_last_error(), MARKER as u64);
        }
    }
}

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
fn format_message_checks_guest_buffers_and_local_allocation() {
    for (wide, lib) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, base) = prepare(wide);
        let output = base + 64;
        emu.maps.write_wide_string(base, "A🌱%n%0discard");
        emu.maps.fill_range(output, 0xcc, 32);
        let flags = u64::from(FORMAT_MESSAGE_FROM_STRING);
        assert_eq!(
            call(
                &mut emu,
                lib,
                "FormatMessageW",
                &[flags, base, 0, 0, output, 5, 0]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_dword(output), Some(MARKER));
        assert_eq!(
            call(
                &mut emu,
                lib,
                "FormatMessageW",
                &[flags, base, 0, 0, output, 6, 0]
            ),
            5
        );
        assert_eq!(emu.maps.read_wide_string(output), "A🌱\r\n");
        assert_eq!(emu.maps.read_dword(output + 12), Some(MARKER));
        emu.maps.fill_range(output, 0xcc, 16);
        let flags = u64::from(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER);
        let count = call(
            &mut emu,
            lib,
            "FormatMessageA",
            &[flags, 0, ERROR_ACCESS_DENIED, 0, output, 1, 0],
        );
        let ptr = if wide {
            emu.maps.read_qword(output).unwrap()
        } else {
            u64::from(emu.maps.read_dword(output).unwrap())
        };
        assert_eq!(emu.maps.read_string(ptr), "Access is denied.\r\n");
        assert_eq!(count, "Access is denied.\r\n".len() as u64);
        assert_eq!(
            emu.maps.read_dword(output + if wide { 8 } else { 4 }),
            Some(MARKER)
        );
        assert_eq!(call(&mut emu, lib, "LocalFree", &[ptr]), 0);
    }
}

#[test]
fn profile_strings_share_the_guest_store_and_respect_output_units() {
    for wide in [false, true] {
        let (mut emu, base) = prepare(wide);
        let dll = "kernel32.dll";
        let alias = "kernelbase.dll";
        let key = base + 64;
        let value = base + 128;
        let out = base + 192;
        emu.maps.write_wide_string(base, "Tests");
        emu.maps.write_wide_string(key, "Name");
        emu.maps.write_wide_string(value, "20🌱");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "WritePrivateProfileStringW",
                &[base, key, value, 0]
            ),
            1
        );
        assert_eq!(
            call(&mut emu, alias, "GetProfileIntW", &[base, key, 77]),
            20
        );
        emu.maps.fill_range(out, 0xcc, 32);
        assert_eq!(
            call(
                &mut emu,
                alias,
                "GetPrivateProfileStringW",
                &[base, key, 0, out, 4, 0]
            ),
            3
        );
        assert_eq!(emu.maps.read_word(out + 6), Some(0));
        assert_eq!(emu.maps.read_dword(out + 8), Some(MARKER));
        assert_eq!(
            call(&mut emu, dll, "GetProfileStringW", &[base, 0, 0, out, 16]),
            5
        );
        assert_eq!(emu.maps.read_wide_string(out), "Name");
        assert_eq!(emu.maps.read_word(out + 10), Some(0));
        let (mut other, other_base) = prepare(wide);
        other.maps.write_string(other_base, "Tests");
        other.maps.write_string(other_base + 64, "Name");
        assert_eq!(
            call(
                &mut other,
                dll,
                "GetProfileIntA",
                &[other_base, other_base + 64, 77]
            ),
            77
        );
        assert_eq!(
            call(&mut emu, alias, "WriteProfileStringW", &[base, key, 0]),
            1
        );
        assert_eq!(call(&mut emu, dll, "GetProfileIntW", &[base, key, 77]), 77);
    }
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

#[test]
fn canonical_string_exports_handle_empty_and_unicode_strings() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let right = data + 128;
        for wide in [false, true] {
            let name = if wide { "lstrcmpiW" } else { "lstrcmpiA" };
            if wide {
                emu.maps.write_wide_string(data, "Alpha");
                emu.maps.write_wide_string(right, "beta");
            } else {
                emu.maps.write_string(data, "Alpha");
                emu.maps.write_string(right, "beta");
            }
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", name, &[data, right]) as u32
                    as i32,
                -1
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", name, &[data, data]),
                0
            );
        }
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetUserDefaultLangID", &[]),
            EN_US_LOCALE as u64
        );
        if is64 {
            for (name, wide, text) in [("lstrcpyA", false, ""), ("lstrcpyW", true, "x😀y")] {
                if wide {
                    emu.maps.write_wide_string(data, text);
                } else {
                    emu.maps.write_string(data, text);
                }
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, "kernel32.dll", name, &[right, data]),
                    right
                );
                let copied = if wide {
                    emu.maps.read_wide_string(right)
                } else {
                    emu.maps.read_string(right)
                };
                assert_eq!(copied, text);
            }
        }
    }
}

#[test]
fn paired_locale_exports_use_stack_arguments_and_bound_output() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        // An afternoon in a leap year makes hardcoded guest-epoch output visible.
        for (i, value) in [2024u16, 2, 0, 29, 15, 4, 5, 0].into_iter().enumerate() {
            emu.maps.write_word(data + i as u64 * 2, value);
        }
        let output = data + 256;
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for name in [
                "GetDateFormatA",
                "GetDateFormatW",
                "GetDateFormatEx",
                "GetTimeFormatA",
                "GetTimeFormatW",
                "GetTimeFormatEx",
            ] {
                let date = name.starts_with("GetDate");
                let ex = name.ends_with("Ex");
                let wide = ex || name.ends_with('W');
                for (ptr, text) in [
                    (0, if date { "1/1/2024" } else { "12:00:00 AM" }),
                    (data, if date { "2/29/2024" } else { "3:04:05 PM" }),
                ] {
                    let required = text.len() as u64 + 1;
                    let mut args = vec![if ex { 0 } else { LOCALE_USER_DEFAULT }, 0, ptr, 0, 0, 0];
                    if date && ex {
                        args.push(0);
                    }
                    assert_eq!(
                        helpers::call_winapi_export(&mut emu, library, name, &args),
                        required
                    );
                    emu.maps.memset(output, 0xa5, 64);
                    args[4] = output;
                    args[5] = required - 1;
                    assert_eq!(
                        helpers::call_winapi_export(&mut emu, library, name, &args),
                        0
                    );
                    assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
                    assert_eq!(emu.maps.read_byte(output), Some(0xa5));
                    args[5] = required;
                    assert_eq!(
                        helpers::call_winapi_export(&mut emu, library, name, &args),
                        required
                    );
                    assert_eq!(
                        if wide {
                            emu.maps.read_wide_string(output)
                        } else {
                            emu.maps.read_string(output)
                        },
                        text
                    );
                    assert_eq!(
                        emu.maps
                            .read_byte(output + required * if wide { 2 } else { 1 }),
                        Some(0xa5)
                    );
                }
            }
        }
    }
    let (mut emu, data) = helpers::prepare_winapi(true);
    let output = data + 256;
    for (name, wide) in [("LCMapStringA", false), ("LCMapStringW", true)] {
        if wide {
            emu.maps.write_wide_string(data, "mixed");
        } else {
            emu.maps.write_string(data, "mixed");
        }
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                name,
                &[
                    LOCALE_USER_DEFAULT,
                    LCMAP_UPPERCASE as u64,
                    data,
                    u32::MAX as u64,
                    output,
                    6
                ]
            ),
            6
        );
        assert_eq!(
            if wide {
                emu.maps.read_wide_string(output)
            } else {
                emu.maps.read_string(output)
            },
            "MIXED"
        );
    }
}

#[test]
fn wsprintf_aw_reads_variadic_slots_without_callee_cleanup() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for wide in [false, true] {
            let format = "%s %u %08X %d";
            let text = if wide { "hello Ж" } else { "hello" };
            let string = data + 0x100;
            let output = data + 0x400;
            if wide {
                emu.maps.write_wide_string(data, format);
                emu.maps.write_wide_string(string, text);
            } else {
                emu.maps.write_string(data, format);
                emu.maps.write_string(string, text);
            }
            let api = if wide { "wsprintfW" } else { "wsprintfA" };
            let arguments = [output, data, string, 7, 42, (-3i32) as u32 as u64];
            let stack = emu.regs().rsp;
            if is64 {
                helpers::call_winapi_export(&mut emu, "user32.dll", api, &arguments);
            } else {
                let words: Vec<u32> = arguments.iter().map(|&arg| arg as u32).collect();
                helpers::call_winapi32(
                    &mut emu,
                    |e| helpers::invoke_winapi_export(e, "user32.dll", api),
                    &words,
                );
                assert_eq!(
                    emu.regs().rsp,
                    stack - (words.len() * size_of::<u32>()) as u64
                );
                for _ in &words {
                    emu.stack_pop32(false);
                }
            }
            let actual = if wide {
                emu.maps.read_wide_string(output)
            } else {
                emu.maps.read_string(output)
            };
            assert_eq!(actual, format!("{text} 7 0000002A -3"));
            assert_eq!(emu.regs().get_eax() as usize, actual.encode_utf16().count());
            assert_eq!(emu.regs().rsp, stack);
        }
    }
}

fn lstr_encodings() -> [(bool, Vec<u8>); 2] {
    [
        (false, b"A\xe9B\0".to_vec()),
        (
            true,
            "A😀B\0".encode_utf16().flat_map(u16::to_le_bytes).collect(),
        ),
    ]
}

#[test]
fn lstr_copy_and_concat_preserve_code_units_and_return_the_destination() {
    for is64 in [false, true] {
        let (mut emu, destination) = prepare(is64);
        let source = destination + 128;
        for (wide, source_bytes) in lstr_encodings() {
            let unit_size = if wide {
                size_of::<u16>()
            } else {
                size_of::<u8>()
            };
            let suffix = if wide { "W" } else { "A" };
            assert!(emu.maps.write_bytes(source, &source_bytes));
            for concatenate in [false, true] {
                let mut expected = Vec::new();
                if concatenate {
                    expected.resize(unit_size, 0);
                    expected[0] = b'P';
                }
                let mut initial = expected.clone();
                initial.resize(initial.len() + unit_size, 0);
                emu.maps.memset(destination, MARKER as u8, 64);
                assert!(emu.maps.write_bytes(destination, &initial));
                expected.extend_from_slice(&source_bytes);
                let api = format!(
                    "{}{}",
                    if concatenate { "lstrcat" } else { "lstrcpy" },
                    suffix
                );
                assert_eq!(
                    call(&mut emu, "kernel32.dll", &api, &[destination, source]),
                    destination
                );
                assert_eq!(
                    emu.maps.read_bytes(destination, expected.len()).as_ref(),
                    expected
                );
                assert_eq!(
                    emu.maps.read_byte(destination + expected.len() as u64),
                    Some(MARKER as u8)
                );
                assert_eq!(
                    emu.maps.read_bytes(source, source_bytes.len()).as_ref(),
                    source_bytes
                );
            }
        }
        for api in ["lstrcpy", "lstrcpyn"] {
            emu.maps.write_string(source, "alias");
            let arguments = if api == "lstrcpy" {
                vec![destination, source]
            } else {
                vec![destination, source, 6]
            };
            assert_eq!(call(&mut emu, "kernel32.dll", api, &arguments), destination);
            assert_eq!(emu.maps.read_string(destination), "alias");
        }
        if !is64 {
            emu.maps.write_string(destination, "head");
            emu.maps.write_string(source, "tail");
            assert_eq!(
                call(&mut emu, "kernel32.dll", "lstrcat", &[destination, source]),
                destination
            );
            assert_eq!(emu.maps.read_string(destination), "headtail");
        }
        for api in ["lstrcpyA", "lstrcpyW", "lstrcatA", "lstrcatW"] {
            assert_eq!(call(&mut emu, "kernel32.dll", api, &[destination, 0]), 0);
        }
    }
}

#[test]
fn lstrcpyn_respects_code_unit_capacity_and_does_not_pad() {
    for (is64, library) in [
        (false, "kernel32.dll"),
        (true, "kernel32.dll"),
        (true, "kernelbase.dll"),
    ] {
        let (mut emu, destination) = prepare(is64);
        let source = destination + 128;
        for (wide, bytes) in lstr_encodings() {
            let unit_size = if wide {
                size_of::<u16>()
            } else {
                size_of::<u8>()
            };
            let source_units = bytes.len() / unit_size - 1;
            let api = if wide { "lstrcpynW" } else { "lstrcpynA" };
            assert!(emu.maps.write_bytes(source, &bytes));
            for capacity in [0u32, 1, 2, 3, 4, 16, u32::MAX] {
                let mut expected = vec![MARKER as u8; 32];
                if capacity != 0 {
                    let copied = source_units.min(capacity as usize - 1) * unit_size;
                    expected[..copied].copy_from_slice(&bytes[..copied]);
                    expected[copied..copied + unit_size].fill(0);
                }
                emu.maps.memset(destination, MARKER as u8, expected.len());
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        api,
                        &[destination, source, capacity as u64]
                    ),
                    destination
                );
                assert_eq!(
                    emu.maps.read_bytes(destination, expected.len()).as_ref(),
                    expected,
                    "{library}!{api}, capacity {capacity}"
                );
            }
        }
    }
}

#[test]
fn lstrlen_counts_code_units_and_preserves_last_error() {
    for (is64, library) in [
        (false, "kernel32.dll"),
        (true, "kernel32.dll"),
        (true, "kernelbase.dll"),
    ] {
        let (mut emu, source) = prepare(is64);
        for (wide, bytes) in lstr_encodings() {
            let unit_size = if wide {
                size_of::<u16>()
            } else {
                size_of::<u8>()
            };
            let api = if wide { "lstrlenW" } else { "lstrlenA" };
            assert!(emu.maps.write_bytes(source, &bytes));
            emu.set_last_error(MARKER as u64);
            assert_eq!(
                call(&mut emu, library, api, &[source]),
                (bytes.len() / unit_size - 1) as u64
            );
            assert_eq!(call(&mut emu, library, api, &[0]), 0);
            assert_eq!(emu.get_last_error(), MARKER as u64);
        }
    }
}

fn call_counted_string(emu: &mut Emu, name: &str, args: &[u64], library: Option<&str>) -> u64 {
    if let Some(library) = library {
        call(emu, library, name, args)
    } else {
        helpers::call_winapi64(emu, |emu| assert!(rtl::dispatch(name, emu)), args)
    }
}

#[test]
fn counted_string_initializers_share_layout_lengths_and_bounds() {
    for (is64, library) in [
        (false, Some("ntdll.dll")),
        (true, Some("ntdll.dll")),
        (false, Some("ntoskrnl.exe")),
        (true, Some("ntoskrnl.exe")),
        (true, None),
    ] {
        let (mut emu, data) = prepare(is64);
        let output = data + 0x100;
        let (size, pointer_offset) = if is64 { (16, 8) } else { (8, 4) };
        // A CJK unit, a surrogate pair, and an unpaired surrogate are four raw units.
        let unicode: Vec<u8> = [0x4e2d_u16, 0xd83d, 0xde00, 0xd800, 0]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect();
        for (name, input, length, terminator) in [
            ("RtlInitUnicodeString", &unicode[..], 8, 2),
            ("RtlInitUnicodeStringEx", &unicode[..], 8, 2),
            ("RtlInitAnsiString", &[0x80, 0xff, 0][..], 2, 1),
            ("RtlInitAnsiStringEx", &[0x80, 0xff, 0][..], 2, 1),
            ("RtlInitString", &b"ABC\0"[..], 3, 1),
            ("RtlInitUTF8String", "😀\0".as_bytes(), 4, 1),
        ] {
            for source in [None, Some(&input[input.len() - terminator..]), Some(input)] {
                emu.maps.fill_range(output - 1, 0xcc, size + 2);
                emu.set_last_error(MARKER as u64);
                emu.regs_mut().rax = MARKER as u64;
                let source_ptr = if let Some(bytes) = source {
                    assert!(emu.maps.write_bytes(data, bytes));
                    data
                } else {
                    0
                };
                let result = call_counted_string(&mut emu, name, &[output, source_ptr], library);
                let expected = if source.is_some_and(|s| s.len() > terminator) {
                    length
                } else {
                    0
                };
                assert_eq!(
                    emu.maps.read_word(output),
                    Some(expected),
                    "{name}, is64={is64}, library={library:?}"
                );
                assert_eq!(
                    emu.maps.read_word(output + 2),
                    Some(if source.is_some() {
                        expected + terminator as u16
                    } else {
                        0
                    })
                );
                let buffer = if is64 {
                    emu.maps.read_qword(output + pointer_offset)
                } else {
                    emu.maps.read_dword(output + pointer_offset).map(u64::from)
                };
                assert_eq!(buffer, Some(source_ptr));
                assert_eq!(emu.maps.read_byte(output - 1), Some(0xcc));
                assert_eq!(emu.maps.read_byte(output + size as u64), Some(0xcc));
                assert_eq!(
                    result,
                    if name.ends_with("Ex") {
                        STATUS_SUCCESS
                    } else {
                        MARKER as u64
                    }
                );
                assert_eq!(emu.get_last_error(), MARKER as u64);
                if let Some(bytes) = source {
                    assert_eq!(emu.maps.read_bytes(data, bytes.len()), bytes);
                }
            }
        }
    }
}

#[test]
fn counted_string_initializers_cap_lengths_and_report_extended_overflow() {
    const SOURCE: u64 = 0x5000_0000;
    const CAPACITY: usize = 0x10002;
    for (is64, library) in [
        (false, Some("ntdll.dll")),
        (true, Some("ntdll.dll")),
        (false, Some("ntoskrnl.exe")),
        (true, Some("ntoskrnl.exe")),
        (true, None),
    ] {
        let (mut emu, output) = prepare(is64);
        emu.maps
            .create_map(
                "counted_string",
                SOURCE,
                CAPACITY as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        for (name, extended_name, unit, max_length) in [
            ("RtlInitUnicodeString", "RtlInitUnicodeStringEx", 2, 65532),
            ("RtlInitAnsiString", "RtlInitAnsiStringEx", 1, 65534),
        ] {
            for (length, expected_length, status) in [
                (max_length, max_length, STATUS_SUCCESS),
                (max_length + unit, 0, STATUS_NAME_TOO_LONG),
            ] {
                emu.maps.fill_range(SOURCE, 1, CAPACITY);
                emu.maps.fill_range(SOURCE + length as u64, 0, unit);
                call_counted_string(&mut emu, name, &[output, SOURCE], library);
                assert_eq!(emu.maps.read_word(output), Some(max_length as u16));
                assert_eq!(
                    emu.maps.read_word(output + 2),
                    Some((max_length + unit) as u16)
                );
                assert_eq!(
                    call_counted_string(&mut emu, extended_name, &[output, SOURCE], library),
                    status
                );
                assert_eq!(emu.maps.read_word(output), Some(expected_length as u16));
                assert_eq!(
                    emu.maps.read_word(output + 2),
                    Some(if status == STATUS_SUCCESS {
                        (max_length + unit) as u16
                    } else {
                        0
                    })
                );
                let buffer = if is64 {
                    emu.maps.read_qword(output + 8)
                } else {
                    emu.maps.read_dword(output + 4).map(u64::from)
                };
                assert_eq!(buffer, Some(SOURCE));
            }
        }
    }
}

#[test]
fn kernel_atoms_share_names_and_keep_table_references_across_snapshots() {
    use crate::serialization::Serialization;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let out = text + 64;
        emu.maps.write_wide_string(text, "MiXeD");
        let id = call(&mut emu, "kernel32.dll", "AddAtomW", &[text]);
        assert!((0xc000..=0xffff).contains(&id));
        emu.maps.write_string(text, "mixed");
        assert_eq!(call(&mut emu, "kernelbase.dll", "AddAtomA", &[text]), id);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GlobalFindAtomA", &[text]),
            0
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "GetAtomNameW", &[id, out, 6]),
            5
        );
        assert_eq!(emu.maps.read_wide_string(out), "MiXeD");
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetAtomNameW", &[id, out, 1]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(call(&mut emu, "kernel32.dll", "DeleteAtom", &[id]), 0);
        assert_eq!(call(&mut emu, "kernel32.dll", "FindAtomA", &[text]), id);
        assert_eq!(call(&mut emu, "kernelbase.dll", "DeleteAtom", &[id]), 0);
        assert_eq!(call(&mut emu, "kernel32.dll", "FindAtomA", &[text]), 0);
        if !is64 {
            let local_atom = call(&mut emu, "kernel32.dll", "AddAtomA", &[text]) as u32;
            assert_ne!(local_atom, 0);
            assert_eq!(
                helpers::call_winapi32(&mut emu, winapi32::kernel32::DeleteAtom, &[local_atom]),
                0
            );
            assert_eq!(call(&mut emu, "kernel32.dll", "FindAtomA", &[text]), 0);
        }
        emu.maps.write_string(text, "#42");
        assert_eq!(call(&mut emu, "kernel32.dll", "AddAtomA", &[text]), 42);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetAtomNameA", &[42, out, 4]),
            3
        );
        assert_eq!(emu.maps.read_string(out), "#42");
    }
}

#[test]
fn kernel_text_exports_return_guest_offsets_and_count_embedded_nuls() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let needle = text + 64;
        let dll = "kernelbase.dll";
        emu.maps.write_wide_string(text, "--AbC--");
        emu.maps.write_wide_string(needle, "bc");
        assert_eq!(call(&mut emu, dll, "StrStrIW", &[text, needle]), text + 6);
        assert_eq!(
            call(&mut emu, dll, "StrChrW", &[text, u64::from(b'C')]),
            text + 8
        );
        assert_eq!(call(&mut emu, dll, "StrChrW", &[text, u64::from(b'x')]), 0);
        emu.maps.write_wide_string(needle, "-");
        assert_eq!(call(&mut emu, dll, "StrTrimW", &[text, needle]), 1);
        assert_eq!(emu.maps.read_wide_string(text), "AbC");
        assert_eq!(call(&mut emu, dll, "StrTrimW", &[text, needle]), 0);
        emu.maps.write_bytes(text, b"a\0b!");
        assert_eq!(call(&mut emu, dll, "CharUpperBuffA", &[text, 3]), 3);
        assert_eq!(
            emu.maps.read_dword(text),
            Some(u32::from_le_bytes(*b"A\0B!"))
        );
    }
}

#[test]
fn kernel_ordinal_and_case_mapping_use_counted_utf16() {
    use crate::windows::constants::nls::{FIND_FROMEND, LCMAP_UPPERCASE};
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let needle = text + 64;
        let out = text + 128;
        emu.maps.write_wide_string(text, "\u{4e2d}A\0a");
        emu.maps.write_wide_string(needle, "a");
        for dll in ["kernel32.dll", "kernelbase.dll"] {
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "FindStringOrdinal",
                    &[u64::from(FIND_FROMEND), text, 4, needle, 1, 1]
                ),
                3
            );
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "FindStringOrdinal",
                    &[0, text, 2, needle, 1, 1]
                ),
                1
            );
            emu.maps.write_bytes(out, &[0xa5; 8]);
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "LCMapStringEx",
                    &[0, u64::from(LCMAP_UPPERCASE), text + 2, 3, out, 2, 0, 0, 0]
                ),
                0
            );
            assert_eq!(emu.maps.read_word(out), Some(0xa5a5));
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "LCMapStringEx",
                    &[0, u64::from(LCMAP_UPPERCASE), text + 2, 3, out, 3, 0, 0, 0]
                ),
                3
            );
            assert_eq!(emu.maps.read_word(out + 4), Some(u16::from(b'A')));
            assert_eq!(emu.maps.read_word(out + 6), Some(0xa5a5));
        }
    }
}

#[test]
fn counted_nls_exports_preserve_terminators_and_result_boundaries() {
    for is64 in [false, true] {
        let (mut emu, src) = prepare(is64);
        let out = src + 128;
        for lib in ["kernel32.dll", "kernelbase.dll"] {
            emu.maps.write_bytes(src, &[0x80, 0]);
            for (code_page, first) in [(CP_WINDOWS_1252, 0x20ac), (CP_IBM437, 0x00c7)] {
                for flags in [0, MB_PRECOMPOSED, MB_PRECOMPOSED | MB_ERR_INVALID_CHARS] {
                    emu.maps.memset(out, 0xa5, 8);
                    let args = [u64::from(code_page), u64::from(flags), src, 2, out, 2];
                    assert_eq!(call(&mut emu, lib, "MultiByteToWideChar", &args), 2);
                    assert_eq!(emu.maps.read_word(out), Some(first));
                    assert_eq!(emu.maps.read_word(out + 2), Some(0));
                    assert_eq!(emu.maps.read_byte(out + 4), Some(0xa5));
                }
            }
            emu.maps.write_bytes(src, &[0xe2, 0x82, 0xac, 0]);
            emu.maps.memset(out, 0xa5, 8);
            let args = [
                CP_UTF8,
                u64::from(MB_ERR_INVALID_CHARS),
                src,
                u64::from(u32::MAX),
                out,
                2,
            ];
            assert_eq!(call(&mut emu, lib, "MultiByteToWideChar", &args), 2);
            assert_eq!(emu.maps.read_word(out), Some(0x20ac));
            assert_eq!(emu.maps.read_word(out + 2), Some(0));
            assert_eq!(emu.maps.read_byte(out + 4), Some(0xa5));
            let mut bad = args;
            bad[2] = 1;
            bad[3] = 1;
            assert_eq!(call(&mut emu, lib, "MultiByteToWideChar", &bad), 0);
            assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
            for name in [
                "GetStringTypeA",
                "GetStringTypeW",
                "GetStringTypeExA",
                "GetStringTypeExW",
            ] {
                if name.ends_with('W') {
                    emu.maps.write_wide_string(src, "A");
                } else {
                    emu.maps.write_string(src, "A");
                }
                emu.maps.memset(out, 0xa5, 8);
                let args = [
                    LOCALE_USER_DEFAULT,
                    u64::from(CT_CTYPE1),
                    src,
                    u64::MAX,
                    out,
                ];
                let first = usize::from(name == "GetStringTypeW");
                assert_eq!(call(&mut emu, lib, name, &args[first..]), 1);
                assert_eq!(
                    emu.maps.read_word(out),
                    Some(C1_DEFINED | C1_ALPHA | C1_UPPER | C1_XDIGIT)
                );
                assert_eq!(emu.maps.read_word(out + 2), Some(C1_DEFINED | C1_CNTRL));
                assert_eq!(emu.maps.read_byte(out + 4), Some(0xa5));
            }
            emu.maps.write_bytes(src, b"A\0\0\0Q\0");
            emu.maps.write_bytes(out, b"A\0\0\0Z\0");
            assert_eq!(
                call(&mut emu, lib, "CompareStringOrdinal", &[src, 3, out, 3, 0]),
                CSTR_LESS_THAN
            );
            assert_eq!(
                call(
                    &mut emu,
                    lib,
                    "CompareStringOrdinal",
                    &[src, u64::MAX, out, u64::MAX, 0]
                ),
                CSTR_EQUAL
            );
        }
    }
}

#[test]
fn account_queries_write_the_declared_width_and_roundtrip_privileges() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::{ERROR_INSUFFICIENT_BUFFER, ERROR_NO_SUCH_PRIVILEGE};
    let dll = "advapi32.dll";
    for is64 in [false, true] {
        let (mut emu, base) = prepare_winapi(is64);
        let size = base + 128;
        let luid = base + 144;
        emu.cfg.user_name = "A\u{1f600}".to_owned();
        emu.maps.write_qword(size, u64::MAX << 32);
        assert_eq!(call(&mut emu, dll, "GetUserNameW", &[base, size]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_dword(size), Some(4));
        assert_eq!(call(&mut emu, dll, "GetUserNameW", &[base, size]), 1);
        assert_eq!(emu.maps.read_wide_string(base), emu.cfg.user_name);
        assert_eq!(emu.maps.read_dword(size + 4), Some(u32::MAX));
        emu.maps.write_string(base, "SeDebugPrivilege");
        assert_eq!(
            call(&mut emu, dll, "LookupPrivilegeValueA", &[0, base, luid]),
            1
        );
        emu.maps.write_dword(size, 32);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "LookupPrivilegeNameW",
                &[0, luid, base, size]
            ),
            1
        );
        assert_eq!(emu.maps.read_wide_string(base), "SeDebugPrivilege");
        emu.maps.write_string(base, "MissingPrivilege");
        assert_eq!(
            call(&mut emu, dll, "LookupPrivilegeValueA", &[0, base, luid]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NO_SUCH_PRIVILEGE);
        let (sid, sid_size, domain, domain_size, sid_use) =
            (base + 256, base + 384, base + 400, base + 512, base + 528);
        let user = emu.cfg.user_name.clone();
        emu.maps.write_wide_string(base, &user);
        emu.maps.write_dword(sid_size, 64);
        emu.maps.write_dword(domain_size, 48);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "LookupAccountNameW",
                &[0, base, sid, sid_size, domain, domain_size, sid_use]
            ),
            1
        );
        emu.maps.write_dword(size, 0);
        emu.maps.write_dword(domain_size, 0);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "LookupAccountSidW",
                &[0, sid, 0, size, 0, domain_size, sid_use]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_dword(size), Some(4));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "LookupAccountSidW",
                &[0, sid, base, size, domain, domain_size, sid_use]
            ),
            1
        );
        assert_eq!(emu.maps.read_wide_string(base), user);
        assert_eq!(emu.maps.read_wide_string(domain), emu.cfg.host_name);
    }
}

#[test]
fn strsafe_counts_wide_units_and_preserves_output_guards() {
    for wide in [false, true] {
        let (mut emu, base) = prepare(wide);
        let src = base + 64;
        let count = base + 128;
        emu.maps.write_wide_string(src, "XYZ");
        emu.maps.fill_range(base, 0xcc, 16);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "StringCbCopyW", &[base, 7, src]),
            HRESULT_INSUFFICIENT_BUFFER
        );
        assert_eq!(emu.maps.read_wide_string(base), "XY");
        assert_eq!(emu.maps.read_word(base + 6), Some(0xcccc));
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "StringCchCopyNW",
                &[base, 2, src, 1]
            ),
            S_OK
        );
        assert_eq!(emu.maps.read_wide_string(base), "X");
        assert_eq!(
            call(&mut emu, "kernel32.dll", "StringCchCatW", &[base, 5, src]),
            S_OK
        );
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "StringCchLengthW",
                &[base, 5, count]
            ),
            S_OK
        );
        assert_eq!(emu.maps.read_dword(count), Some(4));
    }
}
