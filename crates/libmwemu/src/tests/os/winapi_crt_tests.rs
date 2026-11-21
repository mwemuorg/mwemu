use std::fs;

use itertools::iproduct;

use crate::ApiMode;
use crate::emu::Emu;
use crate::serialization::Serialization;
use crate::tests::helpers;
use crate::threading::scheduler::ThreadScheduler;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::ApiDisposition;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::crt::*;

const ERROR_MARKER: u64 = 0x1234;
const CRT_LIBRARIES: [&str; 2] = ["msvcrt.dll", "ucrtbase.dll"];

struct MathCase {
    api: &'static str,
    arguments: &'static [f64],
    expected: f64,
}

#[test]
fn winapi_crt_math_uses_stack_or_xmm_arguments_and_returns() {
    helpers::setup();
    let cases = [
        MathCase {
            api: "sqrt",
            arguments: &[81.0],
            expected: 9.0,
        },
        MathCase {
            api: "floor",
            arguments: &[2.75],
            expected: 2.0,
        },
        MathCase {
            api: "ceil",
            arguments: &[2.25],
            expected: 3.0,
        },
        MathCase {
            api: "fabs",
            arguments: &[-3.5],
            expected: 3.5,
        },
        MathCase {
            api: "log",
            arguments: &[1.0],
            expected: 0.0,
        },
        MathCase {
            api: "log10",
            arguments: &[100.0],
            expected: 2.0,
        },
        MathCase {
            api: "exp",
            arguments: &[0.0],
            expected: 1.0,
        },
        MathCase {
            api: "sin",
            arguments: &[0.0],
            expected: 0.0,
        },
        MathCase {
            api: "cos",
            arguments: &[0.0],
            expected: 1.0,
        },
        MathCase {
            api: "tan",
            arguments: &[0.0],
            expected: 0.0,
        },
        MathCase {
            api: "atan",
            arguments: &[0.0],
            expected: 0.0,
        },
        MathCase {
            api: "pow",
            arguments: &[2.0, 5.0],
            expected: 32.0,
        },
        MathCase {
            api: "fmod",
            arguments: &[7.5, 2.0],
            expected: 1.5,
        },
        MathCase {
            api: "atan2",
            arguments: &[0.0, 1.0],
            expected: 0.0,
        },
    ];
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        for case in &cases {
            const PREVIOUS_X87: f64 = -17.0;
            const GPR_CANARY: u64 = 0x1234_5678;
            let stack = emu.regs().rsp;
            emu.fpu_mut().push_f64(PREVIOUS_X87);
            if is64 {
                emu.regs_mut().xmm0 = case.arguments[0].to_bits() as u128;
                emu.regs_mut().xmm1 =
                    case.arguments.get(1).copied().unwrap_or(999.0).to_bits() as u128;
                helpers::call_winapi_export(
                    &mut emu,
                    "msvcrt.dll",
                    case.api,
                    &[GPR_CANARY, GPR_CANARY],
                );
                assert_eq!(
                    f64::from_bits(emu.regs().xmm0 as u64),
                    case.expected,
                    "{}",
                    case.api
                );
            } else {
                let words = helpers::f64_arguments32(case.arguments);
                helpers::call_winapi32(
                    &mut emu,
                    |e| helpers::invoke_winapi_export(e, "msvcrt.dll", case.api),
                    &words,
                );
                assert_eq!(
                    emu.regs().rsp,
                    stack - (words.len() * size_of::<u32>()) as u64,
                    "{} must leave cdecl arguments",
                    case.api
                );
                assert_eq!(emu.fpu_mut().pop_f64(), case.expected, "{}", case.api);
                for _ in &words {
                    emu.stack_pop32(false);
                }
            }
            assert_eq!(
                emu.fpu_mut().pop_f64(),
                PREVIOUS_X87,
                "{} must preserve caller x87 entries",
                case.api
            );
            assert_eq!(emu.regs().rsp, stack);
        }
    }
}

#[test]
fn winapi_crt_cdecl_leaves_arguments_but_thiscall_discards_them() {
    helpers::setup();
    let (mut emu, data) = helpers::prepare_winapi(false);
    emu.maps.write_string(data, "hello");
    let stack = emu.regs().rsp;
    for (library, api, arguments, expected) in [
        ("msvcrt.dll", "abs", vec![(-5i32) as u32], Some(5)),
        ("msvcrt.dll", "labs", vec![(-7i32) as u32], Some(7)),
        ("msvcrt.dll", "strlen", vec![data as u32], Some(5)),
        ("msvcrt.dll", "printf", vec![data as u32, 123], Some(5)),
        ("msvcrt.dll", "_time64", vec![0], None),
        (
            "kernelbase.dll",
            "_initterm",
            vec![data as u32, data as u32],
            None,
        ),
        (
            "kernelbase.dll",
            "_initterm_e",
            vec![data as u32, data as u32],
            Some(0),
        ),
    ] {
        emu.regs_mut().rdx = u32::MAX as u64;
        let result = helpers::call_winapi32(
            &mut emu,
            |e| helpers::invoke_winapi_export(e, library, api),
            &arguments,
        );
        if let Some(value) = expected {
            assert_eq!(result, value, "{api}");
        }
        assert_eq!(
            emu.regs().rsp,
            stack - (arguments.len() * size_of::<u32>()) as u64,
            "{api}"
        );
        for (index, &value) in arguments.iter().enumerate() {
            assert_eq!(
                emu.maps
                    .read_dword(emu.regs().rsp + (index * size_of::<u32>()) as u64),
                Some(value)
            );
        }
        if api == "_time64" {
            assert_eq!(emu.regs().get_edx(), 0);
        }
        for _ in &arguments {
            emu.stack_pop32(false);
        }
    }
    let target = data + 0x100;
    emu.maps.write_dword(data, 0x1234_5678);
    emu.regs_mut().rcx = target;
    helpers::call_winapi_export(&mut emu, "msvcrt.dll", "??0exception@@QAE@ABV0@@Z", &[data]);
    assert_eq!(emu.regs().get_eax(), target);
    assert_eq!(emu.maps.read_dword(target), Some(0x1234_5678));
}

#[test]
fn winapi_crt_ftol_consumes_x87_and_returns_edx_eax() {
    helpers::setup();
    let (mut emu, _) = helpers::prepare_winapi(false);
    emu.set_api_mode(ApiMode::Strict);
    for ((library, api), import) in iproduct!(
        [
            ("msvcrt.dll", "_ftol"),
            ("msvcrt.dll", "_ftol2"),
            ("msvcrt.dll", "_ftol2_sse"),
            ("ucrtbase.dll", "_ftol"),
        ],
        [false, true]
    ) {
        for value in [4_294_967_297.75, -4_294_967_297.75, f64::NAN] {
            let stack = emu.regs().rsp;
            emu.fpu_mut().push_f64(123.0);
            emu.fpu_mut().push_f64(value);
            call_crt_route(&mut emu, library, api, &[], import);
            let result = ((emu.regs().get_edx() << u32::BITS) | emu.regs().get_eax()) as i64;
            assert_eq!(
                result,
                if value.is_nan() {
                    i64::MIN
                } else {
                    value as i64
                },
                "{api}"
            );
            assert_eq!(emu.fpu_mut().pop_f64(), 123.0);
            assert_eq!(emu.regs().rsp, stack);
        }
    }
}

#[test]
fn crt_frame_handler_fallback_keeps_the_callers_arguments() {
    for (is64, library, import) in
        iproduct!([false, true], ["ucrtbase.dll", "msvcrt.dll"], [false, true])
    {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.set_api_mode(ApiMode::SimulateFail);
        emu.cfg.skip_unimplemented = false;
        let version_specific = if is64 {
            ("__CxxFrameHandler4", 4)
        } else {
            ("_except_handler4_common", 6)
        };
        for (api, count) in [
            ("__CxxFrameHandler", 4),
            ("__CxxFrameHandler3", 4),
            version_specific,
        ] {
            let arguments = [data; 6];
            emu.maps.write_dword(data, 0x1234);
            call_crt_route(&mut emu, library, api, &arguments[..count], import);
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.call.name, api);
            assert_eq!(outcome.disposition, ApiDisposition::SimulatedGeneric);
            assert_eq!(emu.maps.read_dword(data), Some(0x1234));
        }
    }
}

#[test]
fn ntdll_and_libgcc_cdecl_exports_leave_the_caller_arguments() {
    let (mut emu, data) = helpers::prepare_winapi(false);
    let source = data + 0x100;
    let destination = data + 0x200;
    emu.maps.write_string(data, "hello");
    emu.maps.write_string(source, " world");
    emu.maps.write_string(destination, "hello");
    let empty_frame = data + 0x300;
    emu.maps.write_dword(empty_frame, 0);
    let stack = emu.sp();
    for (library, api, arguments) in [
        ("ntdll.dll", "strlen", vec![data as u32]),
        ("ntdll.dll", "stricmp", vec![data as u32, data as u32]),
        (
            "ntdll.dll",
            "strcat",
            vec![destination as u32, source as u32],
        ),
        (
            "libgcc_s_dw2-1.dll",
            "__register_frame_info",
            vec![empty_frame as u32, source as u32],
        ),
        (
            "libgcc_s_dw2-1.dll",
            "__deregister_frame_info",
            vec![empty_frame as u32],
        ),
    ] {
        helpers::call_winapi32(
            &mut emu,
            |emu| helpers::invoke_winapi_export(emu, library, api),
            &arguments,
        );
        if api == "__deregister_frame_info" {
            assert_eq!(emu.regs().get_eax(), 0);
        }
        if library == "libgcc_s_dw2-1.dll" {
            assert!(!emu.maps.is_mapped(0x0040_E198));
            assert_eq!(emu.maps.read_string(source), " world");
        }
        assert!(!emu.api_policy.stopped(), "{library}!{api}");
        assert_eq!(
            emu.sp(),
            stack - (arguments.len() * size_of::<u32>()) as u64,
            "{api}"
        );
        for &value in &arguments {
            assert_eq!(emu.stack_pop32(false), Some(value), "{api}");
        }
        assert_eq!(emu.sp(), stack);
    }
    assert_eq!(emu.maps.read_string(destination), "hello world");
}

fn call_crt(emu: &mut Emu, library: &str, api: &str, arguments: &[u64]) -> u64 {
    call_crt_route(emu, library, api, arguments, false)
}

fn call_crt_route(emu: &mut Emu, library: &str, api: &str, arguments: &[u64], import: bool) -> u64 {
    let handler = |emu: &mut Emu| {
        if !import {
            helpers::invoke_winapi_export(emu, library, api);
        } else if emu.cfg.arch.is_64bits() {
            winapi64::gateway_by_import(emu, library, api, 0);
        } else {
            winapi32::gateway_by_import(emu, library, api, 0);
        }
    };
    let stack = emu.regs().rsp;
    let value = if emu.cfg.arch.is_64bits() {
        helpers::call_winapi64(emu, handler, arguments)
    } else {
        let words: Vec<u32> = arguments.iter().map(|value| *value as u32).collect();
        let result = helpers::call_winapi32(emu, handler, &words);
        assert_eq!(
            emu.regs().rsp,
            stack - (arguments.len() * size_of::<u32>()) as u64
        );
        emu.regs_mut().rsp = stack;
        result as u64
    };
    assert_eq!(emu.regs().rsp, stack);
    assert!(
        !emu.api_policy.stopped(),
        "{api}: {:?}",
        emu.api_policy.last_outcome
    );
    value
}

#[test]
fn crt_wide_searches_use_utf16_units_and_return_guest_pointers() {
    for (is64, library) in iproduct!([false, true], ["msvcrt.dll", "ucrtbase.dll"]) {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let needle = text + 64;
        let other = text + 128;
        emu.maps.write_wide_string(text, "é🍁xé");
        emu.set_last_error(123);
        for (api, unit, offset) in [
            ("wcschr", 0xe9, Some(0)),
            ("wcsrchr", 0xe9, Some(8)),
            ("wcschr", 0xdf41, Some(4)),
            ("wcschr", b'x' as u64, Some(6)),
            ("wcschr", 0, Some(10)),
            ("wcsrchr", 0, Some(10)),
            ("wcsrchr", b'y' as u64, None),
        ] {
            assert_eq!(
                call_crt(&mut emu, library, api, &[text, unit]),
                offset.map_or(0, |n| text + n)
            );
        }
        for (pattern, expected) in [("xé", text + 6), ("", text), ("no", 0)] {
            emu.maps.write_wide_string(needle, pattern);
            assert_eq!(
                call_crt(&mut emu, library, "wcsstr", &[text, needle]),
                expected
            );
        }
        emu.maps.write_wide_string(text, "abababac");
        emu.maps.write_wide_string(needle, "ababac");
        assert_eq!(
            call_crt(&mut emu, library, "wcsstr", &[text, needle]),
            text + 4
        );
        emu.maps.write_wide_string(text, "🍁");
        emu.maps.write_wide_string(other, "🍃");
        for (count, expected) in [(0, 0), (1, 0), (2, -1)] {
            let result = call_crt(&mut emu, library, "wcsncmp", &[text, other, count]) as i32;
            assert_eq!(result.signum(), expected);
        }
        assert_eq!(emu.get_last_error(), 123);
    }
}

#[test]
fn crt_characters_preserve_eof_error_state_and_allocation_ownership() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        let allocations = emu.maps.buffers.len();
        for (value, lower, upper) in [
            (b'A' as u64, b'a' as u64, b'A' as u64),
            (b'z' as u64, b'z' as u64, b'Z' as u64),
            (0, 0, 0),
            (0xc4, 0xc4, 0xc4),
            (u32::MAX as u64, u32::MAX as u64, u32::MAX as u64),
        ] {
            for (api, expected) in [
                ("tolower", lower),
                ("toupper", upper),
                ("towlower", lower as u16 as u64),
                ("towupper", upper as u16 as u64),
            ] {
                emu.set_last_error(ERROR_MARKER);
                assert_eq!(
                    call_crt(&mut emu, library, api, &[value]),
                    expected,
                    "{api}, is64={is64}"
                );
                assert_eq!(emu.get_last_error(), ERROR_MARKER);
            }
        }
        for (api, yes, no) in [
            ("isalpha", b'A', b'1'),
            ("isdigit", b'7', b'x'),
            ("isspace", b'\x0b', b'x'),
            ("isalnum", b'1', b'!'),
            ("ispunct", b'!', b' '),
            ("isupper", b'A', b'a'),
            ("islower", b'a', b'A'),
            ("isxdigit", b'f', b'g'),
        ] {
            for (value, expected) in [
                (yes as u64, true),
                (no as u64, false),
                (u32::MAX as u64, false),
                (0x80, false),
            ] {
                emu.set_last_error(ERROR_MARKER);
                assert_eq!(
                    call_crt(&mut emu, library, api, &[value]) != 0,
                    expected,
                    "{api}"
                );
                assert_eq!(emu.get_last_error(), ERROR_MARKER);
            }
        }
        assert_eq!(emu.maps.buffers.len(), allocations);
    }
}

#[test]
fn crt_errno_and_random_state_follow_runtime_thread_and_snapshot() {
    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        let mut parent_errno = Vec::new();
        for library in CRT_LIBRARIES {
            emu.set_last_error(ERROR_MARKER);
            for api in ["getchar", "_fgetchar"] {
                assert_eq!(call_crt(&mut emu, library, api, &[]), EOF);
                assert_eq!(emu.get_last_error(), ERROR_MARKER);
            }
            let allocations = emu.maps.buffers.len();
            for expected in [41, 18467, 6334, 26500, 19169] {
                assert_eq!(
                    call_crt(&mut emu, library, "rand", &[]),
                    expected,
                    "{library}"
                );
            }
            assert_eq!(emu.maps.buffers.len(), allocations);
            call_crt(&mut emu, library, "srand", &[1]);
            assert_eq!(call_crt(&mut emu, library, "rand", &[]), 41);
            let address = call_crt(&mut emu, library, "_errno", &[]);
            assert_ne!(address, 0);
            assert_eq!(emu.maps.read_dword(address), Some(0));
            assert_eq!(call_crt(&mut emu, library, "_errno", &[]), address);
            assert_eq!(emu.maps.buffers.len(), allocations + 1);
            assert_eq!(
                call_crt(&mut emu, library, "_set_errno", &[(-7i32) as u32 as u64]),
                0
            );
            assert_eq!(emu.maps.read_dword(address), Some((-7i32) as u32));
            assert!(emu.maps.write_dword(address, 73));
            emu.maps.fill_range(output, 0xcc, 8);
            assert_eq!(call_crt(&mut emu, library, "_get_errno", &[output]), 0);
            assert_eq!(emu.maps.read_dword(output), Some(73));
            assert_eq!(emu.maps.read_dword(output + 4), Some(0xcccc_cccc));
            assert_eq!(
                call_crt(&mut emu, library, "_get_errno", &[0]),
                EINVAL as u64
            );
            assert_eq!(
                emu.maps.read_dword(address),
                Some(EINVAL as u32),
                "{library}, is64={is64}"
            );
            assert_eq!(emu.get_last_error(), ERROR_MARKER);
            parent_errno.push(address);
        }
        assert_ne!(parent_errno[0], parent_errno[1]);
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, 0x600000, 0, 0, 0],
        );
        assert_eq!(emu.threads.len(), 2);
        assert!(ThreadScheduler::switch_to_thread(&mut emu, 1));
        let mut child_errno = Vec::new();
        for library in CRT_LIBRARIES {
            assert_eq!(call_crt(&mut emu, library, "rand", &[]), 41);
            let address = call_crt(&mut emu, library, "_errno", &[]);
            assert!(!parent_errno.contains(&address));
            assert_eq!(emu.maps.read_dword(address), Some(0));
            call_crt(&mut emu, library, "_set_errno", &[59]);
            child_errno.push(address);
        }
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        for (thread, addresses, error) in [(0, parent_errno, EINVAL as u32), (1, child_errno, 59)] {
            assert!(ThreadScheduler::switch_to_thread(&mut emu, thread));
            for (library, address) in CRT_LIBRARIES.into_iter().zip(addresses) {
                assert_eq!(call_crt(&mut emu, library, "_errno", &[]), address);
                assert_eq!(emu.maps.read_dword(address), Some(error));
                assert_eq!(call_crt(&mut emu, library, "rand", &[]), 18467);
            }
        }
    }
}

#[test]
fn crt_named_imports_and_api_sets_share_the_ucrt_model() {
    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        call_crt_route(&mut emu, "ucrtbase.dll", "_set_errno", &[73], true);
        for (library, import) in iproduct!(
            [
                "ucrtbase.dll",
                "api-ms-win-crt-runtime-l1-1-0.dll",
                "api-ms-win-crt-string-l1-1-0.dll",
            ],
            [false, true]
        ) {
            assert_eq!(
                call_crt_route(&mut emu, library, "_get_errno", &[output], import),
                0
            );
            assert_eq!(emu.maps.read_dword(output), Some(73));
            assert_eq!(
                call_crt_route(&mut emu, library, "tolower", &[b'A' as u64], import),
                b'a' as u64
            );
        }
        call_crt_route(&mut emu, "msvcrt.dll", "_get_errno", &[output], true);
        assert_eq!(emu.maps.read_dword(output), Some(0));
    }
}

fn seek_crt(emu: &mut Emu, library: &str, file: u64, offset: i64, origin: u64) -> u64 {
    let arguments = if emu.cfg.arch.is_64bits() {
        vec![file, offset as u64, origin]
    } else {
        vec![
            file,
            offset as u32 as u64,
            (offset as u64) >> u32::BITS,
            origin,
        ]
    };
    call_crt(emu, library, "_fseeki64", &arguments)
}

fn tell_crt(emu: &mut Emu, library: &str, file: u64) -> u64 {
    let low = call_crt(emu, library, "_ftelli64", &[file]);
    if emu.cfg.arch.is_64bits() {
        low
    } else {
        (emu.regs().get_edx() << u32::BITS) | low
    }
}

#[test]
fn crt_streams_share_binary_io_seek_widths_and_output_layouts() {
    const FILE_DATA: &[u8] = b"\xff\0A\nrest";
    const FAR_POSITION: i64 = 0x1_0000_0007;
    let host_dir = helpers::file_system_root().join("c/crt-streams");
    fs::create_dir_all(&host_dir).unwrap();
    for (is64, library, wide) in iproduct!([false, true], CRT_LIBRARIES, [false, true]) {
        let filename = format!("stream-{is64}-{library}-{wide}.bin");
        let host = host_dir.join(&filename);
        fs::write(&host, FILE_DATA).unwrap();
        let guest = format!("C:\\crt-streams\\{filename}");
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let mode = data + 0x180;
        let output = data + 0x200;
        let buffer = data + 0x300;
        if wide {
            emu.maps.write_wide_string(data, &guest);
            emu.maps.write_wide_string(mode, "r+b");
        } else {
            emu.maps.write_string(data, &guest);
            emu.maps.write_string(mode, "r+b");
        }
        emu.set_last_error(ERROR_MARKER);
        emu.maps.fill_range(output, 0xcc, 16);
        let open = if wide { "_wfopen_s" } else { "fopen_s" };
        assert_eq!(
            call_crt(&mut emu, library, open, &[output, data, mode]),
            0,
            "{library}, is64={is64}"
        );
        let file = if is64 {
            emu.maps.read_qword(output).unwrap()
        } else {
            emu.maps.read_dword(output).unwrap() as u64
        };
        assert_ne!(file, 0);
        let pointer_size = if is64 { 8 } else { 4 };
        assert_eq!(
            emu.maps.read_dword(output + pointer_size),
            Some(0xcccc_cccc)
        );
        assert_eq!(call_crt(&mut emu, library, "_fileno", &[file]), 3);
        assert_eq!(
            call_crt(&mut emu, library, "_filelength", &[3]),
            FILE_DATA.len() as u64
        );
        assert_eq!(
            call_crt(&mut emu, library, "_filelengthi64", &[3]),
            FILE_DATA.len() as u64
        );
        if !is64 {
            assert_eq!(emu.regs().get_edx(), 0);
        }
        assert_ne!(
            call_crt(&mut emu, library, "_get_osfhandle", &[3]) as u32,
            u32::MAX
        );
        assert_eq!(
            call_crt(
                &mut emu,
                library,
                "_read",
                &[3, buffer, FILE_DATA.len() as u64]
            ),
            FILE_DATA.len() as u64
        );
        assert_eq!(tell_crt(&mut emu, library, file), FILE_DATA.len() as u64);
        call_crt(&mut emu, library, "rewind", &[file]);
        assert_eq!(
            call_crt(
                &mut emu,
                library,
                "fread",
                &[buffer, 1, FILE_DATA.len() as u64, file]
            ),
            FILE_DATA.len() as u64
        );
        assert_eq!(
            emu.maps.read_bytes(buffer, FILE_DATA.len()).as_ref(),
            FILE_DATA
        );
        assert_eq!(tell_crt(&mut emu, library, file), FILE_DATA.len() as u64);
        assert_eq!(call_crt(&mut emu, library, "fgetc", &[file]), EOF);
        assert_ne!(call_crt(&mut emu, library, "feof", &[file]), 0);
        assert_eq!(seek_crt(&mut emu, library, file, -4, SEEK_END), 0);
        assert_eq!(call_crt(&mut emu, library, "feof", &[file]), 0);
        assert_eq!(call_crt(&mut emu, library, "ftell", &[file]), 4);
        assert_eq!(
            call_crt(&mut emu, library, "fgets", &[buffer, 1, file]),
            buffer
        );
        assert_eq!(emu.maps.read_byte(buffer), Some(0));
        assert_eq!(tell_crt(&mut emu, library, file), 4);
        assert_eq!(seek_crt(&mut emu, library, file, FAR_POSITION, SEEK_SET), 0);
        assert_eq!(tell_crt(&mut emu, library, file), FAR_POSITION as u64);
        assert_eq!(call_crt(&mut emu, library, "ftell", &[file]), EOF);
        call_crt(&mut emu, library, "rewind", &[file]);
        assert_eq!(tell_crt(&mut emu, library, file), 0);
        assert!(emu.maps.write_bytes(buffer, b"Z\0"));
        assert_eq!(
            call_crt(&mut emu, library, "fwrite", &[buffer, 1, 2, file]),
            2
        );
        call_crt(&mut emu, library, "rewind", &[file]);
        assert_eq!(call_crt(&mut emu, library, "_write", &[3, buffer, 2]), 2);
        assert_eq!(call_crt(&mut emu, library, "fflush", &[file]), 0);
        assert_eq!(fs::read(&host).unwrap(), b"Z\0A\nrest");
        for api in ["fread", "fwrite"] {
            assert_eq!(call_crt(&mut emu, library, api, &[0, 0, u64::MAX, 0]), 0);
            assert_eq!(
                call_crt(&mut emu, library, api, &[buffer, u64::MAX, 2, file]),
                0
            );
            assert_eq!(tell_crt(&mut emu, library, file), 2);
        }
        assert_eq!(call_crt(&mut emu, library, "fclose", &[file]), 0);
        assert!(!emu.maps.buffers.contains_key(&file));
        assert_eq!(call_crt(&mut emu, library, "fclose", &[file]), EOF);
        assert_eq!(
            call_crt(&mut emu, library, open, &[output, 0, mode]),
            EINVAL as u64
        );
        assert_eq!(emu.maps.read_dword(output), Some(0));
        assert_eq!(
            emu.maps.read_dword(output + pointer_size),
            Some(0xcccc_cccc)
        );
        assert_eq!(emu.get_last_error(), ERROR_MARKER);
    }
}

#[test]
fn crt_streams_belong_to_the_emulator_and_runtime() {
    let host_dir = helpers::file_system_root().join("c/crt-streams");
    fs::create_dir_all(&host_dir).unwrap();
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let mut emulators = Vec::new();
        for (index, bytes) in [b"first".as_slice(), b"second".as_slice()]
            .into_iter()
            .enumerate()
        {
            let filename = format!("owner-{is64}-{library}-{index}.bin");
            fs::write(host_dir.join(&filename), bytes).unwrap();
            let (mut emu, data) = helpers::prepare_winapi(is64);
            emu.maps
                .write_string(data, &format!("C:\\crt-streams\\{filename}"));
            emu.maps.write_string(data + 0x180, "rb");
            let file = call_crt(&mut emu, library, "fopen", &[data, data + 0x180]);
            assert_ne!(file, 0);
            emulators.push((emu, data, file));
        }
        assert_eq!(
            emulators[0].2, emulators[1].2,
            "exercise equal guest addresses in independent emulators"
        );
        let (mut first, _, first_file) = emulators.remove(0);
        let (mut second, data, second_file) = emulators.remove(0);
        assert_eq!(call_crt(&mut first, library, "fclose", &[first_file]), 0);
        assert_eq!(
            call_crt(&mut second, library, "fgetc", &[second_file]),
            b's' as u64
        );
        let other = if library == "msvcrt.dll" {
            "ucrtbase.dll"
        } else {
            "msvcrt.dll"
        };
        assert_eq!(call_crt(&mut second, other, "fclose", &[second_file]), EOF);
        assert_eq!(
            call_crt(&mut second, library, "fgetc", &[second_file]),
            b'e' as u64
        );
        assert!(second.maps.write_byte(data, b'x'));
        assert_eq!(
            call_crt(&mut second, library, "fwrite", &[data, 1, 1, second_file]),
            0
        );
        assert_ne!(call_crt(&mut second, library, "ferror", &[second_file]), 0);
        assert_eq!(call_crt(&mut second, library, "feof", &[second_file]), 0);
        call_crt(&mut second, library, "clearerr", &[second_file]);
        assert_eq!(call_crt(&mut second, library, "ferror", &[second_file]), 0);
        assert_eq!(call_crt(&mut second, library, "fclose", &[second_file]), 0);
    }
}

#[test]
fn crt_environment_initialization_preserves_pointer_layers_and_runtime_copies() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.maps.write_string(data, "CRT_INITIAL");
        emu.maps.write_string(data + 0x100, "old");
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "SetEnvironmentVariableA",
            &[data, data + 0x100],
        );
        let mut pointers = Vec::new();
        for library in CRT_LIBRARIES {
            emu.set_last_error(ERROR_MARKER);
            assert_eq!(
                call_crt(&mut emu, library, "_initialize_narrow_environment", &[]),
                0
            );
            let initial = call_crt(&mut emu, library, "_get_initial_narrow_environment", &[]);
            let cell = call_crt(&mut emu, library, "__p__environ", &[]);
            assert_ne!(initial, 0);
            assert_ne!(cell, initial);
            assert_eq!(read_pointer(&emu, cell), Some(initial));
            assert_ne!(read_pointer(&emu, initial), Some(0));
            let count = emu.maps.buffers.len();
            assert_eq!(
                call_crt(&mut emu, library, "_get_initial_narrow_environment", &[]),
                initial
            );
            assert_eq!(call_crt(&mut emu, library, "__p__environ", &[]), cell);
            assert_eq!(emu.maps.buffers.len(), count);
            let value = call_crt(&mut emu, library, "getenv", &[data]);
            assert_eq!(emu.maps.read_string(value), "old");
            assert_eq!(emu.get_last_error(), ERROR_MARKER);
            pointers.push((initial, cell));
        }
        assert_ne!(pointers[0].0, pointers[1].0);
        emu.maps.write_string(data + 0x100, "new");
        assert_eq!(
            call_crt(&mut emu, "msvcrt.dll", "_putenv_s", &[data, data + 0x100]),
            0
        );
        let updated = call_crt(&mut emu, "msvcrt.dll", "getenv", &[data]);
        let separate = call_crt(&mut emu, "ucrtbase.dll", "getenv", &[data]);
        assert_eq!(emu.maps.read_string(updated), "new");
        assert_eq!(emu.maps.read_string(separate), "old");
        assert_ne!(read_pointer(&emu, pointers[0].1), Some(pointers[0].0));
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        for (library, (initial, cell)) in CRT_LIBRARIES.into_iter().zip(pointers) {
            assert_eq!(
                call_crt(&mut emu, library, "_get_initial_narrow_environment", &[]),
                initial
            );
            assert_eq!(call_crt(&mut emu, library, "__p__environ", &[]), cell);
        }
        let value = call_crt(&mut emu, "msvcrt.dll", "getenv", &[data]);
        assert_eq!(emu.maps.read_string(value), "new");
    }
}

#[test]
fn crt_environment_narrow_wide_mutation_and_invalid_parameters_share_contracts() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let value = data + 0x100;
        emu.set_last_error(ERROR_MARKER);
        assert_eq!(
            call_crt(&mut emu, library, "_initialize_wide_environment", &[]),
            0
        );
        let wide_cell = call_crt(&mut emu, library, "__p__wenviron", &[]);
        emu.maps.write_wide_string(data, "UNICODE=café €");
        assert_eq!(call_crt(&mut emu, library, "_wputenv", &[data]), 0);
        emu.maps.write_bytes(data, b"unicode\0");
        let ansi = call_crt(&mut emu, library, "getenv", &[data]);
        assert_eq!(emu.maps.read_bytes(ansi, 7).as_ref(), b"caf\xe9 \x80\0");
        emu.maps.write_wide_string(data, "unicode");
        let wide = call_crt(&mut emu, library, "_wgetenv", &[data]);
        assert_eq!(emu.maps.read_wide_string(wide), "café €");
        assert_ne!(read_pointer(&emu, wide_cell), Some(0));
        emu.maps.write_wide_string(value, "");
        assert_eq!(call_crt(&mut emu, library, "_wputenv_s", &[data, value]), 0);
        assert_eq!(call_crt(&mut emu, library, "_wgetenv", &[data]), 0);
        for (api, arguments, expected) in [
            ("_putenv", vec![0], u32::MAX as u64),
            ("_putenv_s", vec![0, 0], EINVAL as u64),
            ("getenv", vec![0], 0),
        ] {
            assert_eq!(call_crt(&mut emu, library, api, &arguments), expected);
            let error = call_crt(&mut emu, library, "_errno", &[]);
            assert_eq!(emu.maps.read_dword(error), Some(EINVAL as u32));
            assert_eq!(emu.get_last_error(), ERROR_MARKER);
        }
    }
}

#[test]
fn crt_descriptors_share_stream_ownership_and_binary_file_effects() {
    let host_dir = helpers::file_system_root().join("c/crt-descriptors");
    fs::create_dir_all(&host_dir).unwrap();
    for (is64, library, wide) in iproduct!([false, true], CRT_LIBRARIES, [false, true]) {
        let filename = format!("descriptor-{is64}-{library}-{wide}.bin");
        let host = host_dir.join(&filename);
        fs::write(&host, b"old").unwrap();
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let path = format!("C:\\crt-descriptors\\{filename}");
        if wide {
            emu.maps.write_wide_string(data, &path);
        } else {
            emu.maps.write_string(data, &path);
        }
        let buffer = data + 0x200;
        let open = if wide { "_wopen" } else { "_open" };
        let errno = call_crt(&mut emu, library, "_errno", &[]);
        emu.set_last_error(ERROR_MARKER);
        let flags = _O_WRONLY | _O_CREAT | _O_TRUNC | _O_BINARY;
        let fd = call_crt(
            &mut emu,
            library,
            open,
            &[data, flags as u64, (_S_IREAD | _S_IWRITE) as u64],
        );
        assert_eq!(fd, 3);
        assert!(emu.maps.write_bytes(buffer, b"A\0\xffB"));
        assert_eq!(call_crt(&mut emu, library, "_write", &[fd, buffer, 4]), 4);
        assert_eq!(call_crt(&mut emu, library, "_read", &[fd, buffer, 1]), EOF);
        assert_eq!(emu.maps.read_dword(errno), Some(EBADF as u32));
        assert_eq!(call_crt(&mut emu, library, "_close", &[fd]), 0);
        assert_eq!(call_crt(&mut emu, library, "_close", &[fd]), EOF);
        assert_eq!(fs::read(&host).unwrap(), b"A\0\xffB");
        assert_eq!(
            call_crt(
                &mut emu,
                library,
                open,
                &[
                    data,
                    (_O_WRONLY | _O_CREAT | _O_EXCL | _O_BINARY) as u64,
                    _S_IWRITE as u64
                ]
            ),
            EOF
        );
        assert_eq!(emu.maps.read_dword(errno), Some(EEXIST as u32));
        let fd = call_crt(
            &mut emu,
            library,
            open,
            &[data, (_O_RDONLY | _O_BINARY) as u64],
        );
        assert_eq!(fd, 3);
        assert!(emu.maps.fill_range(buffer, 0xcc, 8));
        assert_eq!(call_crt(&mut emu, library, "_read", &[fd, buffer, 8]), 4);
        assert_eq!(
            emu.maps.read_bytes(buffer, 8).as_ref(),
            b"A\0\xffB\xcc\xcc\xcc\xcc"
        );
        assert_eq!(call_crt(&mut emu, library, "_read", &[fd, buffer, 8]), 0);
        assert_eq!(call_crt(&mut emu, library, "_write", &[fd, buffer, 1]), EOF);
        assert_eq!(emu.maps.read_dword(errno), Some(EBADF as u32));
        assert_eq!(call_crt(&mut emu, library, "_close", &[fd]), 0);
        let fd = call_crt(
            &mut emu,
            library,
            open,
            &[data, (_O_WRONLY | _O_APPEND | _O_BINARY) as u64],
        );
        assert_eq!(call_crt(&mut emu, library, "_write", &[fd, buffer, 1]), 1);
        assert_eq!(call_crt(&mut emu, library, "_close", &[fd]), 0);
        assert_eq!(fs::read(&host).unwrap(), b"A\0\xffBA");
        assert_eq!(call_crt(&mut emu, library, "_write", &[1, buffer, 4]), 4);
        assert_eq!(emu.emulated_stdout, b"A\0\xffB");
        assert_eq!(call_crt(&mut emu, library, "_close", &[1]), 0);
        assert_eq!(call_crt(&mut emu, library, "_write", &[1, buffer, 4]), EOF);
        let fd = call_crt(
            &mut emu,
            library,
            open,
            &[data, (_O_WRONLY | _O_APPEND | _O_BINARY) as u64],
        );
        assert_eq!(fd, 1, "reuse the lowest released descriptor");
        assert_eq!(call_crt(&mut emu, library, "_write", &[fd, buffer, 1]), 1);
        assert_eq!(call_crt(&mut emu, library, "_close", &[fd]), 0);
        assert_eq!(emu.emulated_stdout, b"A\0\xffB");
        assert_eq!(fs::read(&host).unwrap(), b"A\0\xffBAA");
        assert_eq!(emu.get_last_error(), ERROR_MARKER);
    }
}

#[test]
fn crt_startup_arguments_keep_native_pointer_layers_and_snapshot_state() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.cfg.filename = r"C:\Program Files\fixture.exe".into();
        emu.cfg.arguments = r#""a b" "" c\"d "é""#.into();
        emu.set_last_error(ERROR_MARKER);
        let width = if is64 { 8 } else { 4 };
        let mut saved = Vec::new();
        for library in CRT_LIBRARIES {
            let count = call_crt(&mut emu, library, "__p___argc", &[]);
            assert_eq!(emu.maps.read_dword(count), Some(5));
            for (api, wide) in [("__p___argv", false), ("__p___wargv", true)] {
                let cell = call_crt(&mut emu, library, api, &[]);
                assert_eq!(call_crt(&mut emu, library, api, &[]), cell);
                let table = read_pointer(&emu, cell).unwrap();
                assert_ne!(cell, table);
                for (index, expected) in [r"C:\Program Files\fixture.exe", "a b", "", "c\"d", "é"]
                    .into_iter()
                    .enumerate()
                {
                    let address = read_pointer(&emu, table + index as u64 * width).unwrap();
                    if wide {
                        assert_eq!(emu.maps.read_wide_string(address), expected);
                    } else if expected == "é" {
                        assert_eq!(emu.maps.read_bytes(address, 2).as_ref(), b"\xe9\0");
                    } else {
                        assert_eq!(emu.maps.read_string(address), expected);
                    }
                }
                assert_eq!(read_pointer(&emu, table + 5 * width), Some(0));
                emu.maps.fill_range(data, 0xcc, 32);
                let name = if wide {
                    "__wgetmainargs"
                } else {
                    "__getmainargs"
                };
                assert_eq!(
                    call_crt_route(
                        &mut emu,
                        library,
                        name,
                        &[data, data + 8, data + 16, 0, 0],
                        true
                    ),
                    0
                );
                assert_eq!(emu.maps.read_dword(data), Some(5));
                assert_eq!(emu.maps.read_dword(data + 4), Some(0xcccccccc));
                assert_eq!(read_pointer(&emu, data + 8), Some(table));
                let env = read_pointer(&emu, data + 16).unwrap();
                assert_ne!(read_pointer(&emu, env), Some(0));
                saved.push((library, api, cell));
            }
            assert_eq!(
                call_crt(
                    &mut emu,
                    library,
                    "_configure_narrow_argv",
                    &[CRT_ARGV_UNEXPANDED_ARGUMENTS as u64]
                ),
                0
            );
            assert_eq!(
                call_crt(
                    &mut emu,
                    library,
                    "_configure_wide_argv",
                    &[u32::MAX as u64]
                ),
                EINVAL as u64
            );
            assert_eq!(emu.get_last_error(), ERROR_MARKER);
        }
        assert_ne!(saved[0].2, saved[2].2);
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        for (library, api, cell) in saved {
            assert_eq!(call_crt(&mut emu, library, api, &[]), cell);
        }
    }
}

#[test]
fn crt_argument_parsing_handles_quotes_and_rejects_unmodeled_expansion() {
    for (text, expected) in [
        (r#""a b c" d e"#, vec!["a b c", "d", "e"]),
        (r#"a\\\b d"e f"g h"#, vec![r"a\\\b", "de fg", "h"]),
        (r#"a\\\"b c d"#, vec![r#"a\"b"#, "c", "d"]),
        (r#"a\\\\"b c" d e"#, vec![r"a\\b c", "d", "e"]),
        (r#"a"b"" c d"#, vec!["ab\" c d"]),
    ] {
        let (mut emu, _) = helpers::prepare_winapi(true);
        emu.cfg.arguments = text.into();
        let count = call_crt(&mut emu, "ucrtbase.dll", "__p___argc", &[]);
        assert_eq!(emu.maps.read_dword(count), Some(expected.len() as u32 + 1));
        let cell = call_crt(&mut emu, "ucrtbase.dll", "__p___argv", &[]);
        let table = read_pointer(&emu, cell).unwrap();
        for (index, expected) in expected.into_iter().enumerate() {
            let address = read_pointer(&emu, table + (index as u64 + 1) * 8).unwrap();
            assert_eq!(emu.maps.read_string(address), expected, "{text}");
        }
    }
    let (mut emu, data) = helpers::prepare_winapi(false);
    emu.cfg.arguments = "*.exe".into();
    assert_eq!(
        call_crt(
            &mut emu,
            "ucrtbase.dll",
            "_configure_narrow_argv",
            &[CRT_ARGV_EXPANDED_ARGUMENTS as u64]
        ),
        ENOSYS as u64
    );
    assert!(emu.maps.fill_range(data, 0xcc, 32));
    assert_eq!(
        call_crt(
            &mut emu,
            "msvcrt.dll",
            "__getmainargs",
            &[data, data + 8, data + 16, 1, 0]
        ),
        u32::MAX as u64
    );
    assert_eq!(emu.maps.read_dword(data), Some(0xcccccccc));
}

#[test]
fn standard_streams_share_descriptors_and_reopen_preserves_file_pointer() {
    let dir = helpers::file_system_root().join("c/crt-streams");
    fs::create_dir_all(&dir).unwrap();
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let stdout = call_crt(&mut emu, library, "__acrt_iob_func", &[1]);
        assert_ne!(stdout, 0);
        assert_eq!(call_crt(&mut emu, library, "_fileno", &[stdout]), 1);
        assert_eq!(call_crt(&mut emu, library, "fflush", &[stdout]), 0);
        emu.maps.write_bytes(data, b"msg\xff\0");
        assert_ne!(call_crt(&mut emu, library, "fputs", &[data, stdout]), EOF);
        assert_eq!(emu.emulated_stdout, b"msg\xff");
        let filename = format!("redirect-{is64}-{library}.bin");
        emu.maps
            .write_wide_string(data, &format!("C:\\crt-streams\\{filename}"));
        emu.maps.write_wide_string(data + 256, "wb");
        assert_eq!(
            call_crt(&mut emu, library, "_wfreopen", &[data, data + 256, stdout]),
            stdout
        );
        assert_eq!(call_crt(&mut emu, library, "_fileno", &[stdout]), 1);
        assert_eq!(call_crt(&mut emu, library, "fputc", &[0x1ff, stdout]), 0xff);
        assert_eq!(call_crt(&mut emu, library, "fclose", &[stdout]), 0);
        assert_eq!(fs::read(dir.join(filename)).unwrap(), [0xff]);
        assert_eq!(call_crt(&mut emu, library, "fputc", &[0, stdout]), EOF);
    }
}

#[test]
fn crt_bounded_strings_respect_capacity_count_and_guest_units() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, dest) = helpers::prepare_winapi(is64);
        let src = dest + 64;
        let errno = call_crt(&mut emu, library, "_errno", &[]);
        for (width, raw, copy, counted, append) in [
            (1, 0xe9u16, "strcpy_s", "strncpy_s", "strcat_s"),
            (2, 0xd800u16, "wcscpy_s", "wcsncpy_s", "wcscat_s"),
        ] {
            let unit = &raw.to_le_bytes()[..width];
            let source = [unit, unit, &[0, 0][..width]].concat();
            assert!(emu.maps.write_bytes(src, &source));
            for (name, capacity, input, count, expected, used) in [
                (copy, 3, src, None, 0, 2),
                (copy, 2, src, None, ERANGE, 0),
                (copy, 2, 0, None, EINVAL, 0),
                (counted, 2, src, Some(1), 0, 1),
                (counted, 2, src, Some(2), ERANGE, 0),
                (counted, 2, src, Some(_TRUNCATE), STRUNCATE, 1),
                (counted, 3, src, Some(_TRUNCATE), 0, 2),
                (counted, 1, 0, Some(0), 0, 0),
                (append, 4, src, None, 0, 3),
                (append, 2, src, None, ERANGE, 0),
                (append, 1, src, None, EINVAL, 0),
            ] {
                emu.maps.memset(dest, 0xa5, 16);
                if name == append {
                    emu.maps.write_bytes(dest, unit);
                    if capacity > 1 {
                        emu.maps.write_bytes(dest + width as u64, &[0, 0][..width]);
                    }
                }
                emu.maps.write_dword(errno, 77);
                emu.set_last_error(ERROR_MARKER);
                let mut args = vec![dest, capacity, input];
                args.extend(count);
                assert_eq!(
                    call_crt(&mut emu, library, name, &args),
                    expected as u64,
                    "{library}!{name}"
                );
                assert_eq!(
                    emu.maps.read_bytes(dest, used * width).as_ref(),
                    unit.repeat(used)
                );
                assert_eq!(
                    emu.maps
                        .read_bytes(dest + (used * width) as u64, width)
                        .as_ref(),
                    &[0, 0][..width]
                );
                assert_eq!(
                    emu.maps.read_byte(dest + capacity * width as u64),
                    Some(0xa5)
                );
                assert_eq!(
                    emu.maps.read_dword(errno),
                    Some(if matches!(expected, EINVAL | ERANGE) {
                        expected as u32
                    } else {
                        77
                    })
                );
                assert_eq!(emu.get_last_error(), ERROR_MARKER);
            }
            assert_eq!(call_crt(&mut emu, library, counted, &[0, 0, 0, 0]), 0);
        }
    }
}

#[test]
fn crt_text_floats_use_numeric_prefixes_locales_and_the_float_return_register() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let errno = call_crt(&mut emu, library, "_errno", &[]);
        emu.maps.write_dword(errno, 77);
        let locale = text + 128;
        let info = text + 160;
        let convention = text + 512;
        let decimal = text + 544;
        let legacy = library == "msvcrt.dll";
        let offset = match (legacy, is64) {
            (true, true) => 296,
            (true, false) => 188,
            (false, true) => 248,
            (false, false) => 136,
        };
        for (address, pointer) in [
            (locale, info),
            (info + offset, convention),
            (convention, decimal),
        ] {
            assert!(write_pointer(&mut emu, address, pointer));
        }
        emu.maps.write_string(decimal, ",");
        for (name, wide, localized) in [
            ("atof", false, false),
            ("_atof_l", false, true),
            ("_wtof", true, false),
            ("_wtof_l", true, true),
        ] {
            for (token, loc, expected) in [
                (" \u{b}-12.5e1tail", 0, -125.0),
                ("1e+tail", 0, 1.0),
                ("1D2", 0, if legacy { 100.0 } else { 1.0 }),
                ("infinity", 0, if legacy { 0.0 } else { f64::INFINITY }),
                ("0x1.8p2", 0, if legacy { 0.0 } else { 6.0 }),
                ("0x1p-1074", 0, if legacy { 0.0 } else { f64::from_bits(1) }),
                ("-0", 0, -0.0),
                ("1,25xyz", locale, if localized { 1.25 } else { 1.0 }),
            ] {
                if wide {
                    emu.maps.write_wide_string(text, token);
                } else {
                    emu.maps.write_string(text, token);
                }
                let args = [text, loc];
                emu.set_last_error(ERROR_MARKER);
                emu.regs_mut().xmm0 = 42.0f64.to_bits() as u128;
                call_crt(&mut emu, library, name, &args[..1 + usize::from(localized)]);
                let value = if is64 {
                    f64::from_bits(emu.regs().xmm0 as u64)
                } else {
                    emu.fpu_mut().pop_f64()
                };
                assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{library}!{name}({token})"
                );
                assert_eq!(emu.get_last_error(), ERROR_MARKER);
                assert_eq!(emu.maps.read_dword(errno), Some(77));
            }
        }
    }
}

#[test]
fn formatted_streams_and_standard_descriptors_use_real_outputs() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let output = call_crt(&mut emu, library, "__acrt_iob_func", &[1]);
        let handle = call_crt(&mut emu, library, "_get_osfhandle", &[1]);
        assert_eq!(
            handle,
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetStdHandle", &[0xffff_fff5])
        );
        emu.maps.write_bytes(data, b"%d:%s%%\0");
        emu.maps.write_bytes(data + 64, b"x\xff\0");
        let ptr = data + 128;
        if is64 {
            emu.maps.write_qword(ptr, u32::MAX as u64);
            emu.maps.write_qword(ptr + 8, data + 64);
        } else {
            emu.maps.write_dword(ptr, u32::MAX);
            emu.maps.write_dword(ptr + 4, (data + 64) as u32);
        }
        for api in ["fprintf", "vfprintf", "__stdio_common_vfprintf"] {
            let args = match api {
                "fprintf" => vec![output, data, u32::MAX as u64, data + 64],
                "vfprintf" => vec![output, data, ptr],
                _ if is64 => vec![0, output, data, 0, ptr],
                _ => vec![0, 0, output, data, 0, ptr],
            };
            emu.emulated_stdout.clear();
            assert_eq!(call_crt(&mut emu, library, api, &args), 6);
            assert_eq!(emu.emulated_stdout, b"-1:x\xff%");
        }
    }
}

#[test]
fn crt_named_text_functions_share_cdecl_storage_and_return_width() {
    let dll = "msvcrt.dll";
    for (is64, wide) in iproduct!([false, true], [false, true]) {
        let (mut emu, text) = helpers::prepare_winapi(is64);
        let (lower, upper, lower_s, upper_s, dup, decimal) = if wide {
            (
                "_wcslwr",
                "_wcsupr",
                "_wcslwr_s",
                "_wcsupr_s",
                "_wcsdup",
                "_wtoi64",
            )
        } else {
            (
                "_strlwr",
                "_strupr",
                "_strlwr_s",
                "_strupr_s",
                "_strdup",
                "_atoi64",
            )
        };
        let width = if wide { 2 } else { 1 };
        let encode = |s: &str| -> Vec<u8> {
            if wide {
                s.encode_utf16()
                    .chain(Some(0))
                    .flat_map(u16::to_le_bytes)
                    .collect()
            } else {
                s.bytes().chain(Some(0)).collect()
            }
        };
        emu.maps.write_bytes(text, &encode("AbC"));
        emu.set_last_error(123);
        for (api, expected) in [
            (lower, "abc"),
            (upper, "ABC"),
            (lower_s, "abc"),
            (upper_s, "ABC"),
        ] {
            let safe = api.ends_with("_s");
            let args = if safe { vec![text, 4] } else { vec![text] };
            assert_eq!(
                call_crt_route(&mut emu, dll, api, &args, true),
                if safe { 0 } else { text },
                "{api}"
            );
            assert_eq!(
                emu.maps.read_byte(text + 2 * width),
                Some(expected.as_bytes()[2])
            );
        }
        let copy = call_crt_route(&mut emu, dll, dup, &[text], true);
        assert_ne!(copy, 0);
        assert_ne!(copy, text);
        assert_eq!(emu.maps.read_byte(copy + 2 * width), Some(b'C'));
        call_crt_route(&mut emu, dll, "free", &[copy], true);
        assert_eq!(
            call_crt_route(&mut emu, dll, lower_s, &[text, 2], true),
            EINVAL as u64
        );
        assert_eq!(emu.maps.read_byte(text), Some(0));
        emu.maps.write_bytes(text, &encode(" -4294967297tail"));
        let low = call_crt_route(&mut emu, dll, decimal, &[text], true);
        let result = if is64 {
            low
        } else {
            (emu.regs().rdx << 32) | low
        };
        assert_eq!(result, (-4294967297i64) as u64);
        let message = call_crt_route(&mut emu, dll, "strerror", &[EINVAL as u64], true);
        assert_eq!(emu.maps.read_string(message), "Invalid argument");
        assert_eq!(
            call_crt_route(&mut emu, dll, "strerror", &[ENOENT as u64], true),
            message
        );
        assert_eq!(emu.get_last_error(), 123);
    }
}

#[test]
fn ucrt_imports_and_exports_preserve_crt_widths_state_and_caller_owned_arguments() {
    for (is64, import) in iproduct!([false, true], [false, true]) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let invoke = |emu: &mut Emu, api: &str, args: &[u64]| {
            call_crt_route(emu, "ucrtbase.dll", api, args, import)
        };
        emu.maps.write_bytes(data, &[0xa5; 16]);
        let seconds = invoke(&mut emu, "_time32", &[data]);
        assert_eq!(emu.maps.read_dword(data), Some(seconds as u32));
        assert_eq!(emu.maps.read_dword(data + 4), Some(0xa5a5a5a5));
        emu.regs_mut().rdx = u64::MAX;
        assert_eq!(invoke(&mut emu, "_time64", &[data]), seconds);
        assert_eq!(emu.maps.read_qword(data), Some(seconds));
        if !is64 {
            assert_eq!(emu.regs().get_edx(), 0);
        }
        let pointer = invoke(&mut emu, "__p__fmode", &[]);
        assert_ne!(pointer, 0);
        assert_eq!(invoke(&mut emu, "_set_fmode", &[_O_BINARY as _]), 0);
        assert_eq!(invoke(&mut emu, "__p__fmode", &[]), pointer);
        assert_eq!(emu.maps.read_dword(pointer), Some(_O_BINARY as _));
        assert_eq!(
            invoke(&mut emu, "_set_invalid_parameter_handler", &[data]),
            0
        );
        assert_eq!(
            invoke(&mut emu, "_set_invalid_parameter_handler", &[0]),
            data
        );
        emu.maps.write_bytes(data, b"  -7z\0");
        assert_eq!(
            invoke(&mut emu, "strtol", &[data, data + 32, 10]) as u32,
            (-7i32) as u32
        );
        assert_eq!(read_pointer(&emu, data + 32), Some(data + 4));
        emu.maps.write_bytes(data, &[0x80, 0, 0xff, 0]);
        assert!((invoke(&mut emu, "strcmp", &[data, data + 2]) as i32) < 0);
        let table = data + 64;
        let width = pointer_size(&emu);
        emu.maps.write_bytes(table, &[0xa5; 32]);
        assert_eq!(invoke(&mut emu, "_initialize_onexit_table", &[table]), 0);
        assert_eq!(emu.maps.read_byte(table + width * 3), Some(0xa5));
        assert_eq!(
            invoke(&mut emu, "_register_onexit_function", &[table, data]),
            0
        );
        let first = read_pointer(&emu, table).unwrap();
        assert_eq!(read_pointer(&emu, first), Some(data));
        assert_eq!(read_pointer(&emu, table + width), Some(first + width));
    }
}

#[test]
fn command_line_pointers_agree_across_kernel_dlls_and_crt_accessors() {
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        emu.cfg.filename = "C:\\My Tools\\tést.exe".into();
        emu.cfg.arguments = "-v".into();
        for (api, accessor, wide) in [
            ("GetCommandLineW", "__p__wcmdln", true),
            ("GetCommandLineA", "__p__acmdln", false),
        ] {
            let value = helpers::call_winapi_export(&mut emu, "kernel32.dll", api, &[]);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernelbase.dll", api, &[]),
                value
            );
            let cell = call_crt(&mut emu, "ucrtbase.dll", accessor, &[]);
            assert_eq!(read_pointer(&emu, cell), Some(value));
            let encoding = if wide { Encoding::Wide } else { Encoding::Ansi };
            assert_eq!(
                encoding.read(&emu, value).unwrap(),
                "\"C:\\My Tools\\tést.exe\" -v"
                    .encode_utf16()
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn crt_integer_base_zero_and_token_state_use_guest_pointer_width() {
    for (is64, library) in iproduct!([false, true], CRT_LIBRARIES) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.maps.write_bytes(data, &[0xa5; 16]);
        let seconds = call_crt(&mut emu, library, "_time32", &[data]);
        assert_eq!(emu.maps.read_dword(data), Some(seconds as u32));
        assert_eq!(emu.maps.read_byte(data + 4), Some(0xa5));
        assert_eq!(call_crt(&mut emu, library, "_time64", &[data]), seconds);
        assert_eq!(emu.maps.read_qword(data), Some(seconds));
        emu.maps.write_bytes(data, b"0xff!\0");
        for api in ["strtol", "strtoul"] {
            assert_eq!(call_crt(&mut emu, library, api, &[data, data + 32, 0]), 255);
            assert_eq!(read_pointer(&emu, data + 32), Some(data + 4));
        }
        for api in ["strtok", "strtok_s"] {
            emu.maps.write_bytes(data, b",a,,b\0");
            emu.maps.write_bytes(data + 16, b",\0");
            let args = |text| {
                if api == "strtok_s" {
                    vec![text, data + 16, data + 32]
                } else {
                    vec![text, data + 16]
                }
            };
            assert_eq!(call_crt(&mut emu, library, api, &args(data)), data + 1);
            assert_eq!(call_crt(&mut emu, library, api, &args(0)), data + 4);
            assert_eq!(call_crt(&mut emu, library, api, &args(0)), 0);
        }
    }
}

#[test]
fn crt_termination_runs_callbacks_in_reverse_order_and_keeps_the_runtimes_separate() {
    use crate::maps::mem64::Permission;
    const CODE: u64 = 0x2800_0000;
    const COUNT: u32 = CODE as u32 + 0x100;
    for is64 in [false, true] {
        let (mut emu, table) = helpers::prepare_winapi(is64);
        emu.maps
            .create_map("crt_callbacks", CODE, 4096, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        for (offset, digit) in [(0, 1u8), (32, 2)] {
            // mov eax, COUNT; shl dword ptr [eax], 4; or byte ptr [eax], digit; ret
            let mut body = vec![0xb8];
            body.extend(COUNT.to_le_bytes());
            body.extend([0xc1, 0x20, 4, 0x80, 0x08, digit, 0xc3]);
            emu.maps.write_bytes(CODE + offset, &body);
        }
        emu.regs_mut().rip = CODE + 64;
        emu.maps.write_byte(CODE + 64, 0x90);
        for dll in ["ucrtbase.dll", "msvcrt.dll"] {
            assert_eq!(call_crt(&mut emu, dll, "_crt_atexit", &[CODE]), 0);
            assert_eq!(call_crt(&mut emu, dll, "_onexit", &[CODE + 32]), CODE + 32);
        }
        call_crt(&mut emu, "ucrtbase.dll", "_c_exit", &[]);
        assert_eq!(emu.maps.read_dword(COUNT.into()), Some(0));
        call_crt(&mut emu, "ucrtbase.dll", "_cexit", &[]);
        assert_eq!(emu.maps.read_dword(COUNT.into()), Some(0x21));
        call_crt(&mut emu, "ucrtbase.dll", "_cexit", &[]);
        assert_eq!(emu.maps.read_dword(COUNT.into()), Some(0x21));
        call_crt(&mut emu, "msvcrt.dll", "_cexit", &[]);
        assert_eq!(emu.maps.read_dword(COUNT.into()), Some(0x2121));
        assert_eq!(
            call_crt(
                &mut emu,
                "ucrtbase.dll",
                "_initialize_onexit_table",
                &[table]
            ),
            0
        );
        assert_eq!(
            call_crt(
                &mut emu,
                "ucrtbase.dll",
                "_register_onexit_function",
                &[table, CODE]
            ),
            0
        );
        assert_eq!(
            call_crt(&mut emu, "ucrtbase.dll", "_execute_onexit_table", &[table]),
            0
        );
        assert_eq!(emu.maps.read_dword(COUNT.into()), Some(0x21211));
    }
}

#[test]
fn crt_locale_mode_uses_one_argument_and_float_controls_follow_policy() {
    use crate::windows::constants::crt::{_DISABLE_PER_THREAD_LOCALE, _ENABLE_PER_THREAD_LOCALE};
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for dll in ["msvcrt.dll", "ucrtbase.dll"] {
            let change = |emu: &mut Emu, mode: i32| {
                call_crt(emu, dll, "_configthreadlocale", &[u64::from(mode as u32)]) as i32
            };
            assert_eq!(change(&mut emu, 0), _DISABLE_PER_THREAD_LOCALE);
            assert_eq!(
                change(&mut emu, _ENABLE_PER_THREAD_LOCALE),
                _DISABLE_PER_THREAD_LOCALE
            );
            assert_eq!(change(&mut emu, 0), _ENABLE_PER_THREAD_LOCALE);
            assert_eq!(change(&mut emu, 9), -1);
            assert_eq!(
                change(&mut emu, _DISABLE_PER_THREAD_LOCALE),
                _ENABLE_PER_THREAD_LOCALE
            );
            let (mut strict, _) = helpers::prepare_winapi(is64);
            strict.set_api_mode(ApiMode::Strict);
            let handler = |emu: &mut Emu| helpers::invoke_winapi_export(emu, dll, "_controlfp_s");
            strict.maps.write_dword(data, 0xcafebabe);
            if is64 {
                helpers::call_winapi64(&mut strict, handler, &[data, 0, 0]);
            } else {
                helpers::call_winapi32(&mut strict, handler, &[data as u32, 0, 0]);
            }
            assert!(strict.api_policy.stopped());
            assert_eq!(strict.maps.read_dword(data), Some(0xcafebabe));
        }
    }
}

#[test]
fn counted_printf_respects_c_and_msvcrt_truncation_rules() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let output = data + 128;
        emu.maps.write_string(data, "%d!");
        for lib in ["msvcrt.dll", "ucrtbase.dll"] {
            for (api, capacity, result, bytes) in [
                ("snprintf", 0, 4, &b""[..]),
                ("snprintf", 3, 4, &b"12\0"[..]),
                ("_snprintf", 3, u32::MAX as u64, &b"123"[..]),
                ("_snprintf", 4, 4, &b"123!"[..]),
                ("_snprintf", 5, 4, &b"123!\0"[..]),
            ] {
                emu.maps.memset(output, 0xa5, 16);
                let args = [output, capacity, data, 123];
                let value = if is64 {
                    helpers::call_winapi64(
                        &mut emu,
                        |e| helpers::invoke_winapi_export(e, lib, api),
                        &args,
                    )
                } else {
                    let stack = emu.sp();
                    let value = helpers::call_winapi32(
                        &mut emu,
                        |e| helpers::invoke_winapi_export(e, lib, api),
                        &args.map(|v| v as u32),
                    );
                    assert_eq!(emu.sp(), stack - 16);
                    for _ in args {
                        emu.stack_pop32(false);
                    }
                    u64::from(value)
                };
                assert_eq!(value as u32, result as u32, "{lib}!{api}");
                assert_eq!(emu.maps.read_bytes(output, bytes.len()).as_ref(), bytes);
                assert_eq!(emu.maps.read_byte(output + bytes.len() as u64), Some(0xa5));
            }
        }
    }
}
