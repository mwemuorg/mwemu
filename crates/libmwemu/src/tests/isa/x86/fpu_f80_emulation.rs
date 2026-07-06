use crate::fpu::f80::F80;

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

    // Test infinity
    f80.set_f64(f64::INFINITY);
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
