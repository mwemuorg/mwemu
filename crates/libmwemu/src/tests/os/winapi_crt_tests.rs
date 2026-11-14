use crate::tests::helpers;

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
            Some(0),
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
    for api in ["_ftol", "_ftol2", "_ftol2_sse"] {
        for value in [4_294_967_297.75, -4_294_967_297.75, f64::NAN] {
            let stack = emu.regs().rsp;
            emu.fpu_mut().push_f64(123.0);
            emu.fpu_mut().push_f64(value);
            helpers::invoke_winapi_export(&mut emu, "msvcrt.dll", api);
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
