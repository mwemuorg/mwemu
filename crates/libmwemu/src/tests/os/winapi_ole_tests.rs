use crate::api::windows::common::memory::{read_pointer, write_pointer};
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;

use crate::*;

// ─── ole32 64-bit ───

#[test]
fn co_initialize_ex_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoInitializeEx, &[0, 0]);
    assert_eq!(ret, 0, "CoInitializeEx should return S_OK (0)");
}

#[test]
fn co_task_mem_alloc_64() {
    helpers::setup();
    let mut emu = emu64();
    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc should return non-zero pointer");
    assert!(
        emu.maps.is_mapped(ptr),
        "CoTaskMemAlloc pointer should be mapped"
    );
}

#[test]
fn co_task_mem_free_64() {
    helpers::setup();
    let mut emu = emu64();
    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x100]);
    assert_ne!(ptr, 0);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);
    assert_eq!(ret, 0, "CoTaskMemFree should return 0");
}

#[test]
fn co_task_mem_free_null_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[0]);
    assert_eq!(ret, 0, "CoTaskMemFree(NULL) should not crash");
}

#[test]
fn com_apartments_count_matching_initializations_per_thread_across_restore() {
    use crate::serialization::Serialization;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        let dll = "ole32.dll";
        let original = emu.current_thread().id;
        assert_eq!(call(&mut emu, dll, "CoInitialize", &[0]), S_OK);
        assert_eq!(call(&mut emu, dll, "OleInitialize", &[0]), S_OK);
        assert_eq!(call(&mut emu, dll, "OleInitialize", &[0]), S_FALSE);
        call(&mut emu, dll, "OleUninitialize", &[]);
        call(&mut emu, dll, "OleUninitialize", &[]);
        call(&mut emu, dll, "OleUninitialize", &[]);
        assert_eq!(
            call(&mut emu, dll, "CoInitializeEx", &[0, 0]),
            RPC_E_CHANGED_MODE
        );
        call(&mut emu, dll, "CoUninitialize", &[]);
        assert_eq!(call(&mut emu, dll, "OleInitialize", &[0]), S_OK);
        assert_eq!(call(&mut emu, dll, "CoInitialize", &[0]), S_FALSE);
        assert_eq!(
            call(&mut emu, dll, "CoInitializeEx", &[0, 0]),
            RPC_E_CHANGED_MODE
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        emu.current_thread_mut().id = original + 1;
        assert_eq!(call(&mut emu, dll, "CoInitializeEx", &[0, 0]), S_OK);
        emu.current_thread_mut().id = original;
        call(&mut emu, dll, "OleUninitialize", &[]);
        assert_eq!(
            call(&mut emu, dll, "CoInitializeEx", &[0, 0]),
            RPC_E_CHANGED_MODE
        );
        call(&mut emu, dll, "CoUninitialize", &[]);
        assert_eq!(call(&mut emu, dll, "CoInitializeEx", &[0, 0]), S_OK);
    }
}

#[test]
fn co_task_mem_alloc_free_cycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x100]);
    let p2 = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(p1, 0);
    assert_ne!(p2, 0);
    assert_ne!(p1, p2, "two allocations should differ");

    emu.maps.write_dword(p1, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(p1).unwrap(), 0xCAFEBABE);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[p1]);
    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[p2]);
}

// ─── oleaut32 64-bit ───

#[test]
fn sys_alloc_string_len_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 32]);
    assert_ne!(bstr, 0, "SysAllocStringLen should return non-zero BSTR");
    assert!(emu.maps.is_mapped(bstr), "BSTR pointer should be mapped");
}

#[test]
fn sys_free_string_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 16]);
    assert_ne!(bstr, 0);
    helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysFreeString, &[bstr]);
}

#[test]
fn variant_clear_64() {
    helpers::setup();
    let mut emu = emu64();

    let variant = 0x30000u64;
    emu.maps
        .create_map("variant", variant, 0x1000, Permission::READ_WRITE);
    const VT_UI4: u16 = 19;
    emu.maps.write_word(variant, VT_UI4);
    emu.maps.write_qword(variant + 8, 0xCAFEBABE);

    let ret = helpers::call_winapi64(&mut emu, winapi64::oleaut32::VariantClear, &[variant]);
    assert_eq!(ret, 0, "VariantClear should return S_OK (0)");

    let vt = emu.maps.read_word(variant).unwrap();
    assert_eq!(vt, 0, "VariantClear should set vt to VT_EMPTY (0)");
}

// ─── oleaut32 new functions 64-bit ───

#[test]
fn sys_alloc_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("wide_buf", buf, 0x1000, Permission::READ_WRITE);
    // Write "Hi" as wide string (H=0x48, i=0x69, null=0x00)
    emu.maps.write_word(buf, 0x48);
    emu.maps.write_word(buf + 2, 0x69);
    emu.maps.write_word(buf + 4, 0x00);

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocString, &[buf]);
    assert_ne!(bstr, 0, "SysAllocString should return non-zero BSTR");

    // Read length prefix (4 bytes before BSTR pointer)
    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap();
    assert_eq!(
        len_bytes, 4,
        "BSTR length prefix should be 4 bytes (2 chars)"
    );
}

#[test]
fn sys_string_len_64() {
    helpers::setup();
    let mut emu = emu64();

    // First allocate a BSTR with SysAllocStringLen (known to work)
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 10]);
    assert_ne!(bstr, 0);

    // Now read the length back
    let len = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysStringLen, &[bstr]);
    assert_eq!(len, 10, "SysStringLen should return 10");
}

#[test]
fn sys_string_byte_len_64() {
    helpers::setup();
    let mut emu = emu64();

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 10]);
    assert_ne!(bstr, 0);

    let byte_len = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysStringByteLen, &[bstr]);
    assert_eq!(
        byte_len, 20,
        "SysStringByteLen should return 20 (10 chars * 2)"
    );
}

#[test]
fn variant_init_64() {
    helpers::setup();
    let mut emu = emu64();

    let variant = 0x30000u64;
    emu.maps
        .create_map("variant", variant, 0x1000, Permission::READ_WRITE);
    // Fill with non-zero data
    for i in 0..16u64 {
        emu.maps.write_byte(variant + i, 0xFF);
    }

    helpers::call_winapi64(&mut emu, winapi64::oleaut32::VariantInit, &[variant]);

    // All 16 bytes should be zero
    for i in 0..16u64 {
        let b = emu.maps.read_byte(variant + i).unwrap();
        assert_eq!(b, 0, "VariantInit should zero byte at offset {}", i);
    }
}

#[test]
fn com_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoInitializeEx,
        &[0, COINIT_APARTMENTTHREADED as u64],
    );
    assert_eq!(ret, 0, "CoInitializeEx must return S_OK");

    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "CoTaskMemAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xBAADF00D);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xBAADF00D);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
}

#[test]
fn automation_conversions_use_typed_arguments_and_exact_output_widths() {
    struct Conversion {
        api: &'static str,
        value: f64,
        single: bool,
        output: Vec<u8>,
    }
    helpers::setup();
    let cases = [
        Conversion {
            api: "VarI2FromR8",
            value: 2.5,
            single: false,
            output: 2i16.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarI4FromR8",
            value: 3.5,
            single: false,
            output: 4i32.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarI8FromR8",
            value: -4_294_967_297.0,
            single: false,
            output: (-4_294_967_297i64).to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarUI1FromR4",
            value: 254.5,
            single: true,
            output: vec![254],
        },
        Conversion {
            api: "VarBoolFromR8",
            value: -1.5,
            single: false,
            output: (-1i16).to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarR4FromR8",
            value: 1.25,
            single: false,
            output: 1.25f32.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarR8FromR4",
            value: 1.5,
            single: true,
            output: 1.5f64.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarCyFromR8",
            value: 1.25,
            single: false,
            output: 12500i64.to_le_bytes().to_vec(),
        },
    ];
    const CANARY: u8 = 0xa5;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for case in &cases {
            emu.maps.fill_range(data, CANARY, 32);
            let arguments = if is64 {
                emu.regs_mut().xmm0 = if case.single {
                    (case.value as f32).to_bits() as u128
                } else {
                    case.value.to_bits() as u128
                };
                vec![0x1234, data]
            } else {
                let mut words: Vec<u64> = if case.single {
                    vec![(case.value as f32).to_bits() as u64]
                } else {
                    helpers::f64_arguments32(&[case.value])
                        .into_iter()
                        .map(u64::from)
                        .collect()
                };
                words.push(data);
                words
            };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "oleaut32.dll", case.api, &arguments),
                S_OK,
                "{}",
                case.api
            );
            for (index, &byte) in case.output.iter().enumerate() {
                assert_eq!(
                    emu.maps.read_byte(data + index as u64),
                    Some(byte),
                    "{}",
                    case.api
                );
            }
            assert_eq!(
                emu.maps.read_byte(data + case.output.len() as u64),
                Some(CANARY),
                "{} output overrun",
                case.api
            );
        }
        emu.maps.fill_range(data, CANARY, 16);
        let arguments = if is64 {
            emu.regs_mut().xmm0 = f64::NAN.to_bits() as u128;
            vec![0, data]
        } else {
            let mut words: Vec<u64> = helpers::f64_arguments32(&[f64::NAN])
                .into_iter()
                .map(u64::from)
                .collect();
            words.push(data);
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarI4FromR8", &arguments),
            DISP_E_OVERFLOW
        );
        assert_eq!(
            emu.maps.read_dword(data),
            Some(u32::from_le_bytes([CANARY; 4]))
        );
    }
}

#[test]
fn automation_mixed_fp_arguments_math_and_dates() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let power_arguments = if is64 {
            emu.regs_mut().xmm0 = 2.0f64.to_bits() as u128;
            emu.regs_mut().xmm1 = 5.0f64.to_bits() as u128;
            vec![0, 0, data]
        } else {
            let mut words: Vec<u64> = helpers::f64_arguments32(&[2.0, 5.0])
                .into_iter()
                .map(u64::from)
                .collect();
            words.push(data);
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarR8Pow", &power_arguments),
            S_OK
        );
        assert_eq!(emu.maps.read_qword(data), Some(32.0f64.to_bits()));
        let compare_arguments = if is64 {
            emu.regs_mut().xmm0 = 1.5f32.to_bits() as u128;
            emu.regs_mut().xmm1 = 2.5f64.to_bits() as u128;
            vec![0, 0]
        } else {
            let mut words = vec![1.5f32.to_bits() as u64];
            words.extend(helpers::f64_arguments32(&[2.5]).into_iter().map(u64::from));
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarR4CmpR8", &compare_arguments),
            0
        );
        for (value, year, month, day, hour) in [(2.5, 1900, 1, 1, 12), (-1.25, 1899, 12, 29, 6)] {
            emu.maps.fill_range(data, 0xa5, 32);
            let arguments = if is64 {
                emu.regs_mut().xmm0 = f64::to_bits(value) as u128;
                vec![0, data]
            } else {
                let mut words: Vec<u64> = helpers::f64_arguments32(&[value])
                    .into_iter()
                    .map(u64::from)
                    .collect();
                words.push(data);
                words
            };
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "oleaut32.dll",
                    "VariantTimeToSystemTime",
                    &arguments
                ),
                1
            );
            assert_eq!(emu.maps.read_word(data), Some(year));
            assert_eq!(emu.maps.read_word(data + 2), Some(month));
            assert_eq!(emu.maps.read_word(data + 6), Some(day));
            assert_eq!(emu.maps.read_word(data + 8), Some(hour));
            assert_eq!(emu.maps.read_word(data + 16), Some(0xa5a5));
            let mut arguments = arguments;
            arguments.insert(arguments.len() - 1, 0);
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "oleaut32.dll",
                    "VarUdateFromDate",
                    &arguments
                ),
                S_OK
            );
            let ordinal = emu.maps.read_word(data + 16).expect("UDATE ordinal");
            assert!((1..=366).contains(&ordinal));
        }
    }
}

#[test]
fn com_activation_failures_preserve_iids_and_use_native_output_layouts() {
    const MARKER: u8 = 0xcc;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        let stride = if is64 { 24 } else { 12 };
        let class = data;
        let iid = data + 0x20;
        let results = data + 0x100;
        let source = data + 0x300;
        emu.maps.write_wide_string(source, "C:\\unregistered.dat");
        let original_source = emu.maps.read_bytes(source, 64).to_vec();
        for (api, args, expected) in [
            (
                "CoCreateInstanceEx",
                vec![class, 0, 1, 0, 2, results],
                REGDB_E_CLASSNOTREG,
            ),
            (
                "CoCreateInstanceFromApp",
                vec![class, 0, 1, 0, 2, results],
                REGDB_E_CLASSNOTREG,
            ),
            (
                "CoGetInstanceFromFile",
                vec![0, class, 0, 1, 0, source, 2, results],
                E_NOTIMPL,
            ),
            (
                "CoGetInstanceFromIStorage",
                vec![0, class, 0, 1, source, 2, results],
                E_NOTIMPL,
            ),
        ] {
            assert!(emu.maps.write_bytes(results - 1, &[MARKER; 64]));
            for index in 0..2 {
                let entry = results + index * stride;
                assert!(write_pointer(&mut emu, entry, iid));
                assert!(write_pointer(&mut emu, entry + width, 0));
                assert!(emu.maps.write_dword(entry + 2 * width, 0));
            }
            emu.set_last_error(MARKER as u64);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ole32.dll", api, &args),
                expected,
                "{api}, is64={is64}"
            );
            for index in 0..2 {
                let entry = results + index * stride;
                assert_eq!(read_pointer(&emu, entry), Some(iid));
                assert_eq!(read_pointer(&emu, entry + width), Some(0));
                assert_eq!(
                    emu.maps.read_dword(entry + 2 * width),
                    Some(expected as u32)
                );
                if is64 {
                    assert_eq!(
                        emu.maps.read_dword(entry + 20),
                        Some(u32::from_le_bytes([MARKER; 4]))
                    );
                }
            }
            assert_eq!(emu.maps.read_byte(results - 1), Some(MARKER));
            assert_eq!(emu.maps.read_byte(results + 2 * stride), Some(MARKER));
            assert_eq!(emu.maps.read_bytes(source, 64).as_ref(), original_source);
            assert_eq!(emu.get_last_error(), MARKER as u64);
        }
        assert!(write_pointer(&mut emu, results, u64::MAX));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "ole32.dll",
                "CoCreateInstance",
                &[class, 0, 1, iid, results]
            ),
            REGDB_E_CLASSNOTREG
        );
        assert_eq!(read_pointer(&emu, results), Some(0));
    }
}

#[test]
fn com_activation_rejects_invalid_result_arrays_before_writing() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let results = data + 0x100;
        let width = if is64 { 8 } else { 4 };
        let stride = if is64 { 24 } else { 12 };
        for (count, invalid_iid, expected) in [
            (0, false, HRESULT_E_INVALID_ARG),
            (2, true, E_POINTER),
            (u32::MAX, false, E_POINTER),
        ] {
            assert!(emu.maps.write_bytes(results, &[0xcc; 64]));
            for index in 0..2 {
                assert!(write_pointer(
                    &mut emu,
                    results + index * stride,
                    if invalid_iid && index == 1 { 1 } else { data }
                ));
                assert!(write_pointer(&mut emu, results + index * stride + width, 0));
                assert!(
                    emu.maps
                        .write_dword(results + index * stride + 2 * width, 0)
                );
            }
            let before = emu.maps.read_bytes(results, 64).to_vec();
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "ole32.dll",
                    "CoCreateInstanceEx",
                    &[data, 0, 1, 0, count as u64, results]
                ),
                expected
            );
            assert_eq!(emu.maps.read_bytes(results, 64).as_ref(), before);
        }
    }
}

#[test]
fn automation_integer_conversions_use_exact_ranges_and_abi_slots() {
    let marker = 0x55;
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        for (api, bits, wide_input, expected) in [
            (
                "VarI4FromUI8",
                i32::MAX as u64,
                true,
                Some(i32::MAX.to_le_bytes().to_vec()),
            ),
            ("VarI4FromUI8", 1u64 << 32, true, None),
            (
                "VarI4FromCy",
                15_000,
                true,
                Some(2i32.to_le_bytes().to_vec()),
            ),
            (
                "VarI4FromCy",
                25_000,
                true,
                Some(2i32.to_le_bytes().to_vec()),
            ),
            (
                "VarI4FromCy",
                (-25_000i64) as u64,
                true,
                Some((-2i32).to_le_bytes().to_vec()),
            ),
            ("VarUI1FromCy", 2_555_000, true, None),
            (
                "VarBoolFromCy",
                1,
                true,
                Some((-1_i16).to_le_bytes().to_vec()),
            ),
            ("VarUI1FromI2", (-1_i16) as u64, false, None),
            (
                "VarUI1FromBool",
                (-1_i16) as u64,
                false,
                Some(vec![u8::MAX]),
            ),
            (
                "VarUI8FromI8",
                i64::MAX as u64,
                true,
                Some(i64::MAX.to_le_bytes().to_vec()),
            ),
            (
                "VarBoolFromUI8",
                u64::MAX,
                true,
                Some((-1_i16).to_le_bytes().to_vec()),
            ),
            (
                "VarDecFromBool",
                2,
                false,
                Some(vec![
                    marker, marker, 0, 0x80, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                ]),
            ),
            ("VarCyFromI8", 922_337_203_685_478, true, None),
            (
                "VarCyFromI4",
                (-12_i32) as u64,
                false,
                Some((-120_000_i64).to_le_bytes().to_vec()),
            ),
            (
                "VarR8FromUI8",
                u64::MAX,
                true,
                Some((u64::MAX as f64).to_le_bytes().to_vec()),
            ),
            (
                "VarDecFromI8",
                i64::MIN as u64,
                true,
                Some(vec![
                    marker, marker, 0, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
                ]),
            ),
        ] {
            emu.maps.fill_range(out, marker, 20);
            let args = if wide_input && !is64 {
                vec![bits as u32 as u64, bits >> u32::BITS, out]
            } else {
                vec![bits, out]
            };
            let status = helpers::call_winapi_export(&mut emu, "oleaut32.dll", api, &args);
            assert_eq!(
                status,
                if expected.is_some() {
                    S_OK
                } else {
                    DISP_E_OVERFLOW
                },
                "{api}({bits})"
            );
            let expected = expected.unwrap_or_default();
            assert_eq!(
                emu.maps.read_bytes(out, expected.len()).as_ref(),
                expected,
                "{api}"
            );
            assert_eq!(
                emu.maps.read_byte(out + expected.len() as u64),
                Some(marker),
                "{api}"
            );
        }
    }
}

#[test]
fn automation_fixed_point_uses_even_rounding_and_preserves_variant_tags() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        let dll = "oleaut32.dll";
        for (name, a, b, b_wide, status, result) in [
            ("VarCyRound", 15_000, 0, false, S_OK, 20_000),
            ("VarCyRound", 25_000, 0, false, S_OK, 20_000),
            ("VarCyInt", -15_000, 0, false, S_OK, -20_000),
            ("VarCyFix", -15_000, 0, false, S_OK, -10_000),
            ("VarCyMul", -15_000, 3_333, true, S_OK, -5_000),
            ("VarCyMulI8", i64::MAX, 2, true, DISP_E_OVERFLOW, -2),
            ("VarCyAdd", i64::MAX, 1, true, DISP_E_OVERFLOW, 123),
        ] {
            let mut args = Vec::new();
            let mut append = |value: i64, wide: bool| {
                args.push(value as u64);
                if wide && !is64 {
                    args.push((value as u64) >> u32::BITS);
                }
            };
            append(a, true);
            if !matches!(name, "VarCyInt" | "VarCyFix") {
                append(b, b_wide);
            }
            args.push(output);
            assert!(emu.maps.write_qword(output, 123));
            assert_eq!(call(&mut emu, dll, name, &args), status);
            assert_eq!(emu.maps.read_qword(output), Some(result as u64), "{name}");
        }
        for (name, digits, magnitude, scale, sign) in [
            ("VarDecNeg", 0, 25, 1, 0),
            ("VarDecAbs", 0, 25, 1, 0),
            ("VarDecRound", 0, 2, 0, 0x80),
            ("VarDecInt", 0, 3, 0, 0x80),
            ("VarDecFix", 0, 2, 0, 0x80),
        ] {
            // The output overlaps the input as it can inside a VARIANT.
            let mut raw = [0u8; 16];
            raw[..4].copy_from_slice(&[14, 0, 1, 0x80]);
            raw[8] = 25;
            assert!(emu.maps.write_bytes(output, &raw));
            let args = if name == "VarDecRound" {
                vec![output, digits, output]
            } else {
                vec![output, output]
            };
            assert_eq!(call(&mut emu, dll, name, &args), S_OK);
            raw[2] = scale;
            raw[3] = sign;
            raw[8] = magnitude;
            assert_eq!(emu.maps.read_bytes(output, 16).as_ref(), &raw, "{name}");
        }
    }
}

#[test]
fn com_identifiers_round_trip_with_native_layout_and_task_ownership() {
    use crate::maps::buffers::BufferKind;
    use crate::winapi::common::buffers;
    use crate::winapi::registry_store as registry;
    use helpers::call_winapi_export as call;
    const TEXT: &str = "{01234567-89AB-CDEF-0123-456789ABCDEF}";
    const GUID: [u8; 16] = [
        0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef,
    ];
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let guid = data + 0x100;
        let out = data + 0x200;
        for (dll, api, error) in [
            ("ole32.dll", "CLSIDFromString", CO_E_CLASSSTRING),
            ("ole32.dll", "IIDFromString", CO_E_IIDSTRING),
            ("shell32.dll", "SHCLSIDFromString", CO_E_CLASSSTRING),
        ] {
            emu.maps.write_wide_string(data, &TEXT.to_lowercase());
            assert_eq!(call(&mut emu, dll, api, &[data, guid]), S_OK, "{api}");
            assert_eq!(emu.maps.read_bytes(guid, 16).as_ref(), GUID);
            emu.maps
                .write_wide_string(data, &TEXT.replacen('-', ":", 1));
            assert_eq!(call(&mut emu, dll, api, &[data, guid]), error, "{api}");
            emu.maps.write_wide_string(data, &format!("{TEXT}!"));
            let expected = match api {
                "SHCLSIDFromString" => S_OK,
                "IIDFromString" => HRESULT_E_INVALID_ARG,
                _ => CO_E_CLASSSTRING,
            };
            assert_eq!(call(&mut emu, dll, api, &[data, guid]), expected, "{api}");
            assert_eq!(call(&mut emu, dll, api, &[0, guid]), S_OK);
            assert_eq!(emu.maps.read_bytes(guid, 16).as_ref(), [0; 16]);
        }
        let progid = if is64 {
            "MwEmu.Guid.Test64"
        } else {
            "MwEmu.Guid.Test32"
        };
        let root = registry::predefined_path(registry::HKEY_CLASSES_ROOT).unwrap();
        let clsid = registry::join(root, &format!("{progid}\\CLSID"));
        emu.registry.create_key(&clsid).unwrap();
        assert!(emu.registry.set_value(&clsid, "", registry::sz(TEXT)));
        emu.maps.write_wide_string(data, progid);
        assert_eq!(
            call(&mut emu, "ole32.dll", "CLSIDFromString", &[data, guid]),
            S_OK
        );
        assert_eq!(emu.maps.read_bytes(guid, 16).as_ref(), GUID);
        assert!(
            emu.registry
                .delete_key(&registry::join(root, progid), true)
                .is_ok()
        );

        assert!(emu.maps.write_bytes(out, &[0xcc; 80]));
        assert_eq!(
            call(&mut emu, "ole32.dll", "StringFromGUID2", &[guid, out, 38]),
            0
        );
        assert_eq!(emu.maps.read_byte(out), Some(0xcc));
        assert_eq!(
            call(&mut emu, "ole32.dll", "StringFromGUID2", &[guid, out, 39]),
            39
        );
        assert_eq!(emu.maps.read_wide_string(out), TEXT);
        assert_eq!(emu.maps.read_word(out + 78), Some(0xcccc));
        for api in ["StringFromCLSID", "StringFromIID"] {
            emu.maps.write_qword(out, u64::MAX);
            assert_eq!(call(&mut emu, "ole32.dll", api, &[guid, out]), S_OK);
            let address = read_pointer(&emu, out).unwrap();
            assert_eq!(emu.maps.read_wide_string(address), TEXT);
            assert_eq!(buffers::size(&emu, address, BufferKind::Task), Some(78));
            call(&mut emu, "ole32.dll", "CoTaskMemFree", &[address]);
            assert_eq!(buffers::size(&emu, address, BufferKind::Task), None);
        }
        assert_eq!(call(&mut emu, "ole32.dll", "IsEqualGUID", &[guid, guid]), 1);
        assert_eq!(call(&mut emu, "ole32.dll", "IsEqualGUID", &[guid, data]), 0);
    }
}

#[test]
fn scalar_safe_arrays_follow_guest_layout_bounds_and_lock_ownership() {
    use crate::winapi::common::memory;
    use crate::windows::constants::errors::{DISP_E_ARRAYISLOCKED, DISP_E_BADINDEX};
    const VT_UI1: u64 = 17;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let call = |emu: &mut _, name: &str, args: &[u64]| {
            helpers::call_winapi_export(emu, "oleaut32.dll", name, args)
        };
        let array = call(
            &mut emu,
            "SafeArrayCreateVector",
            &[VT_UI1, (-2i32) as u64, 3],
        );
        assert_ne!(array, 0);
        assert_eq!(call(&mut emu, "SafeArrayGetDim", &[array]), 1);
        assert_eq!(call(&mut emu, "SafeArrayGetElemsize", &[array]), 1);
        for (api, expected) in [("SafeArrayGetLBound", -2i32), ("SafeArrayGetUBound", 0)] {
            assert_eq!(call(&mut emu, api, &[array, 1, data]), S_OK);
            assert_eq!(emu.maps.read_dword(data), Some(expected as u32));
        }
        let index = data + 16;
        let output = data + 32;
        emu.maps.write_dword(index, (-1i32) as u32);
        emu.maps.write_word(data, 0xaa11);
        assert_eq!(
            call(&mut emu, "SafeArrayPutElement", &[array, index, data]),
            S_OK
        );
        emu.maps.write_word(output, 0xcc00);
        assert_eq!(
            call(&mut emu, "SafeArrayGetElement", &[array, index, output]),
            S_OK
        );
        assert_eq!(emu.maps.read_word(output), Some(0xcc11));
        emu.maps.write_dword(index, 1);
        assert_eq!(
            call(&mut emu, "SafeArrayGetElement", &[array, index, output]),
            DISP_E_BADINDEX
        );
        assert_eq!(
            call(&mut emu, "SafeArrayAccessData", &[array, output]),
            S_OK
        );
        let bytes = memory::read_pointer(&emu, output).unwrap();
        assert_eq!(emu.maps.read_bytes(bytes, 3).as_ref(), [0, 0x11, 0]);
        assert_eq!(
            call(&mut emu, "SafeArrayDestroy", &[array]),
            DISP_E_ARRAYISLOCKED
        );
        assert_eq!(call(&mut emu, "SafeArrayUnaccessData", &[array]), S_OK);
        assert_eq!(call(&mut emu, "SafeArrayDestroy", &[array]), S_OK);
    }
}

#[test]
fn automation_strings_use_the_locale_slot_and_return_owned_bstrs() {
    use crate::winapi::common::memory;
    use crate::windows::constants::errors::DISP_E_TYPEMISMATCH;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let call = |emu: &mut _, api: &str, args: &[u64]| {
            helpers::call_winapi_export(emu, "oleaut32.dll", api, args)
        };
        let output = data + 128;
        emu.maps.write_wide_string(data, "127.5");
        emu.maps.fill_range(output, 0xcc, 8);
        assert_eq!(
            call(&mut emu, "VarUI1FromStr", &[data, 0x0409, 0, output]),
            S_OK
        );
        assert_eq!(emu.maps.read_word(output), Some(0xcc80));
        emu.maps.write_wide_string(data, "invalid");
        assert_eq!(
            call(&mut emu, "VarI4FromStr", &[data, 0x0409, 0, output]),
            DISP_E_TYPEMISMATCH
        );
        assert_eq!(emu.maps.read_word(output), Some(0xcc80));
        let args = if is64 {
            vec![u64::MAX, 0x0409, 0, output]
        } else {
            vec![u64::from(u32::MAX), u64::from(u32::MAX), 0x0409, 0, output]
        };
        assert_eq!(call(&mut emu, "VarBstrFromUI8", &args), S_OK);
        let string = memory::read_pointer(&emu, output).unwrap();
        assert_eq!(emu.maps.read_wide_string(string), u64::MAX.to_string());
        assert_eq!(call(&mut emu, "SysStringLen", &[string]), 20);
        let slot = data + 0x200;
        assert!(memory::write_pointer(&mut emu, slot, string));
        assert_eq!(call(&mut emu, "SysReAllocString", &[slot, 0]), 1);
        assert_eq!(memory::read_pointer(&emu, slot), Some(0));
        assert_eq!(call(&mut emu, "SysReAllocStringLen", &[slot, 0, 0]), 1);
        let blank = memory::read_pointer(&emu, slot).unwrap_or(0);
        assert_ne!(blank, 0);
        assert_eq!(call(&mut emu, "SysStringLen", &[blank]), 0);
        call(&mut emu, "SysFreeString", &[blank]);
    }
}

#[test]
fn empty_com_error_info_clears_a_guest_pointer_on_both_export_paths() {
    use crate::winapi::policy::ApiMode;
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        let expected = if is64 { 0 } else { u64::from(u32::MAX) << 32 };
        for dll in ["ole32.dll", "oleaut32.dll"] {
            emu.set_api_mode(ApiMode::SimulateFail);
            assert_eq!(call(&mut emu, dll, "SetErrorInfo", &[0, output]), E_NOTIMPL);
            assert_eq!(call(&mut emu, dll, "SetErrorInfo", &[0, 0]), S_OK);
            assert_eq!(
                call(&mut emu, dll, "SetErrorInfo", &[1, 0]),
                HRESULT_E_INVALID_ARG
            );
            for (reserved, status) in [(0, S_FALSE), (1, HRESULT_E_INVALID_ARG)] {
                assert!(emu.maps.write_qword(output, u64::MAX));
                assert_eq!(
                    call(&mut emu, dll, "GetErrorInfo", &[reserved, output]),
                    status
                );
                assert_eq!(emu.maps.read_qword(output), Some(expected));
            }
        }
    }
}
