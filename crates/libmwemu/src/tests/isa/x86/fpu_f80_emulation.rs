use crate::fpu::f80::{F80, INT_BIT_MASK, REAL_INDEFINITE, SIGN_MASK};
use crate::maps::mem64::Permission;
use crate::{emu32, emu64};

#[test]
#[allow(clippy::approx_constant)]
pub fn fpu_f80_emulation() {
    let mut f80 = F80::new();

    f80.st = (16383u128 << 64) | (1u128 << 63);
    assert_eq!(f80.to_integer_u128(), 1);

    f80.set_f64(1.0);
    assert_eq!(f80.get(), 0x3fff8000000000000000);

    // Test zero
    f80.set_f64(0.0);
    assert!(f80.is_zero());
    assert_eq!(f80.get_f64(), 0.0);

    f80.set_f64(-0.0);
    assert!(f80.is_zero());
    assert_eq!(f80.get_f64(), -0.0);

    for bits in [
        1,
        2,
        f64::MIN_POSITIVE.to_bits() - 1,
        f64::MIN_POSITIVE.to_bits(),
    ] {
        for sign in [0, (-0.0f64).to_bits()] {
            f80.set_f64(f64::from_bits(sign | bits));
            assert!(f80.bit_integer());
            assert_eq!(f80.get_f64().to_bits(), sign | bits);
        }
    }

    // Test infinity
    f80.set_f64(f64::INFINITY);
    assert_eq!(f80.get(), 0x7fff_8000_0000_0000_0000);
    assert!(f80.is_infinite());
    assert_eq!(f80.get_f64(), f64::INFINITY);

    f80.set_f64(f64::NEG_INFINITY);
    assert!(f80.is_infinite());
    assert_eq!(f80.get_f64(), f64::NEG_INFINITY);

    // Test NaN
    f80.set_f64(f64::NAN);
    assert!(f80.is_nan());
    assert!(f80.get_f64().is_nan());

    // Test normal numbers roundtrip with tolerance
    let test_values = [
        1.0,
        -1.0,
        3.141592653589793,
        -2.718281828459045,
        1e10,
        -1e-10,
    ];

    for &val in &test_values {
        f80.set_f64(val);
        let back = f80.get_f64();
        let diff = (val - back).abs();
        assert!(diff < 1e-12, "val: {}, got: {}", val, back);
    }

    // Test flags negative checks
    f80.set_f64(42.0);
    assert!(!f80.is_nan());
    assert!(!f80.is_infinite());
    assert!(!f80.is_zero());

    let test_values = [
        0u128,
        1,
        9,
        10,
        42,
        12345,
        99999999,
        12345678901234567890u128, // big num
    ];

    for &val in &test_values {
        f80.set(val);

        // Conver to BCD packed and reconstruct
        let bcd = f80.to_bcd_packed();
        let mut f80_2 = F80::new();
        f80_2.from_bcd_packed(&bcd);

        assert_eq!(
            f80.to_integer_u128(),
            f80_2.to_integer_u128(),
            "BCD roundtrip: valor entero no coincide para valor {}",
            val
        );
        assert!(
            (f80.get_f64() - f80_2.get_f64()).abs() < 1e-10,
            "BCD roundtrip no coincide para valor {}",
            val
        );
    }

    f80.set_f64(259.0);
    let bcd = f80.to_bcd_packed();

    assert_eq!(bcd.len(), 10);
    assert_eq!(bcd[0], 0x59);
    assert_eq!(bcd[1], 0x02);

    f80.st = F80::encode_from_u128(259, false);
    let bcd = f80.to_bcd_packed();

    assert_eq!(bcd[0], 0x59);
    assert_eq!(bcd[1], 0x02);

    let mut f80 = F80::new();
    let val: u128 = 256;
    f80.set(val);

    let bytes = f80.get_bytes();
    let mut f80_2 = F80::new();
    f80_2.set_bytes(&bytes);

    assert_eq!(f80.get(), f80_2.get(), "Error en get() para valor {}", val);
    assert_eq!(
        f80.to_integer_u128(),
        f80_2.to_integer_u128(),
        "Error en to_integer_u128 para valor {}",
        val
    );

    let bcd1 = f80.to_bcd_packed();
    let bcd2 = f80_2.to_bcd_packed();
    assert_eq!(bcd1, bcd2, "Error en BCD packed para valor {}", val);

    // test a.add(b)

    let mut b: F80 = F80::new();
    f80.set_f64(-1.1);
    b.set_f64(1.9);
    f80.add(b);
    assert_eq!(f80.get_f64(), 0.7999999999999998);
    assert_eq!(f80.get_round_f64(4), 0.8);
    assert_eq!(f80.get(), 0x3ffeccccccccccccc000);

    f80.set_f64(1.0);
    b.set_f64(2.0);
    f80.sub(b);
    assert_eq!(f80.get_f64(), -1.0);
}

#[test]
fn extended_sums_cover_large_gaps_cancellation_and_rounding() {
    let one = 0x3fff_u128 << 64 | INT_BIT_MASK;
    let half_ulp = (0x3fff_u128 - 64) << 64 | INT_BIT_MASK;
    let infinity = 0x7fff_u128 << 64 | INT_BIT_MASK;
    let cases = [
        (one, 0, one),
        (one, SIGN_MASK, one),
        (one, 1, one),
        (one, one | SIGN_MASK, 0),
        (one, half_ulp, one),
        (one + 1, half_ulp, one + 2),
        (
            INT_BIT_MASK << 1 | INT_BIT_MASK,
            (INT_BIT_MASK - 1) | SIGN_MASK,
            1,
        ),
        (infinity, one, infinity),
        (infinity, infinity | SIGN_MASK, REAL_INDEFINITE),
    ];
    for (a, b, expected) in cases {
        for (left, right) in [(a, b), (b, a)] {
            let mut sum = F80 { st: left };
            sum.add(F80 { st: right });
            assert_eq!(sum.get(), expected, "{left:#x} + {right:#x}");
            let mut difference = F80 { st: left };
            difference.sub(F80 {
                st: right ^ SIGN_MASK,
            });
            assert_eq!(difference.get(), expected, "{left:#x} - (-{right:#x})");
        }
    }
}

#[test]
fn x87_integer_loads_preserve_stack_order_and_all_significand_bits() {
    const CODE: u64 = 0x400000;
    const DATA: u64 = 0x500000;
    const SIGNED_PART: u64 = 0x4234_5678_1234_5679;
    for mut emu in [emu32(), emu64()] {
        let bytes = [
            0xd9, 0x2a, // fldcw [edx]
            0xd9, 0xe8, // fld1
            0xdf, 0x69, 8, // fild qword [ecx+8]
            0xdf, 0x29, // fild qword [ecx]
            0xd8, 0xc2, // fadd st,st(2)
            0xde, 0xc1, // faddp st(1),st
            0xd9, 0xc0, // fld st(0)
            0xdf, 0xe0, // fnstsw ax
            0xdd, 0x7a, 2, // fnstsw word [edx+2]
            0xdf, 0x41, 16, // fild word [ecx+16]
            0xdb, 0x41, 20, // fild dword [ecx+20]
        ];
        emu.maps
            .create_map("x87_code", CODE, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        emu.maps
            .create_map("x87_data", DATA, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_bytes(CODE, &bytes);
        emu.maps.write_qword(DATA, i64::MAX as u64);
        emu.maps.write_qword(DATA + 8, SIGNED_PART);
        emu.maps.write_word(DATA + 16, (-3i16).cast_unsigned());
        emu.maps.write_dword(DATA + 20, (-5i32).cast_unsigned());
        emu.maps.write_word(DATA + 32, 0x0f7f); // extended precision, round toward zero
        emu.regs_mut().rcx = DATA;
        emu.regs_mut().rdx = DATA + 32;
        emu.regs_mut().rax = 0xabba_ffff;
        emu.set_pc(CODE);
        for _ in 0..4 {
            assert!(emu.step());
        }
        assert_eq!(emu.fpu().get_ctrl(), 0x0f7f);
        assert_eq!(emu.fpu_mut().get_top(), 5);
        assert_eq!(
            emu.fpu_mut().get_st_u80(0),
            F80::encode_from_u128(i64::MAX as u128, false)
        );
        assert!(emu.step());
        assert!(emu.step());
        let expected = F80::encode_from_u128((1 << 63) + u128::from(SIGNED_PART), false);
        assert_eq!(emu.fpu_mut().get_st_u80(0), expected);
        assert!(emu.step());
        assert_eq!(emu.fpu_mut().get_st_u80(1), expected);
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.regs().rax, 0xabba_2800);
        assert_eq!(emu.maps.read_word(DATA + 34), Some(0x2800));
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.fpu_mut().get_st(0), -5.0);
        assert_eq!(emu.fpu_mut().get_st(1), -3.0);
    }
}

#[test]
fn x87_radix_loop_writes_the_native_digits_and_terminates() {
    const CODE: u64 = 0x400000;
    const DATA: u64 = 0x500000;
    for mut emu in [emu32(), emu64()] {
        emu.maps
            .create_map("radix_code", CODE, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        emu.maps
            .create_map("radix_data", DATA, 0x1000, Permission::READ_WRITE)
            .unwrap();
        let bytes = [
            0xd9, 0xc1, // fld st(1)
            0xd9, 0xf8, // fprem
            0xdf, 0x1b, // fistp word [ebx]
            0xdc, 0xf9, // fdiv st(1),st
            0xd9, 0xc1, // fld st(1)
            0xd8, 0xd3, // fcom st(3)
            0xdf, 0xe0, // fnstsw ax
            0x9e, // sahf
            0x73, 0xf1, // jae fprem
        ];
        emu.maps.write_bytes(CODE, &bytes);
        emu.regs_mut().rbx = DATA;
        for (input, radix, digits) in [
            (23u64, 10, vec![3u16, 2]),
            (
                0xc279_ed78_e6a4_8b62,
                16,
                vec![2, 6, 11, 8, 4, 10, 6, 14, 8, 7, 13, 14, 9, 7, 2, 12],
            ),
            (u64::MAX, 16, vec![15; 16]),
        ] {
            emu.fpu_mut().clear();
            emu.fpu_mut().set_ctrl(0x0f7f);
            emu.fpu_mut().push_f64(1.0);
            emu.fpu_mut().push_f80(F80 {
                st: F80::encode_from_u128(u128::from(input), false),
            });
            emu.fpu_mut().push_f64(f64::from(radix));
            emu.set_pc(CODE);
            assert!(emu.step());
            for (index, digit) in digits.iter().enumerate() {
                assert!(emu.step()); // fprem
                assert!(!emu.fpu().get_status_c2());
                assert!(emu.step()); // fistp
                assert_eq!(emu.maps.read_word(DATA), Some(*digit));
                for _ in 0..6 {
                    assert!(emu.step());
                }
                let branched = emu.pc() == CODE + 2;
                assert_eq!(branched, index + 1 < digits.len(), "digit {index}");
            }
            assert_eq!(emu.pc(), CODE + bytes.len() as u64);
        }
    }
}
