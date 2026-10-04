//! Integer, exclusive/acquire-release and LSE atomic instructions that
//! clang/gcc emit routinely. Encodings come from `clang -target arm64` +
//! `objdump -d`; the mnemonic is next to each word.
use super::{data_map, emu_with, step_ok};

#[test]
fn eon_xors_with_inverted_operand() {
    let mut emu = emu_with(&[0xca220020]); // eon x0, x1, x2
    emu.regs_aarch64_mut().x[1] = 0xF0F0;
    emu.regs_aarch64_mut().x[2] = 0x0FF0;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xFFFF_FFFF_FFFF_00FF);
}

#[test]
fn bics_sets_zero_and_negative_flags() {
    let mut emu = emu_with(&[0xea220020, 0xea220020]); // bics x0, x1, x2 (x2)
    emu.regs_aarch64_mut().x[1] = 0xFF;
    emu.regs_aarch64_mut().x[2] = 0xFF;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0);
    assert!(emu.regs_aarch64().nzcv.z);
    assert!(!emu.regs_aarch64().nzcv.n);

    emu.regs_aarch64_mut().x[1] = 1 << 63;
    emu.regs_aarch64_mut().x[2] = 0;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 1 << 63);
    assert!(emu.regs_aarch64().nzcv.n);
    assert!(!emu.regs_aarch64().nzcv.z);
}

#[test]
fn bfxil_replaces_low_bits_only() {
    let mut emu = emu_with(&[0xb3483c20]); // bfxil x0, x1, #8, #8
    emu.regs_aarch64_mut().x[0] = 0xAAAA_AAAA_AAAA_AAAA;
    emu.regs_aarch64_mut().x[1] = 0x1234_5678_9ABC_DEF0;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xAAAA_AAAA_AAAA_AADE);
}

#[test]
fn bfi_inserts_field_and_clears_upper_word() {
    let mut emu = emu_with(&[0x331c1c20]); // bfi w0, w1, #4, #8
    emu.regs_aarch64_mut().x[0] = 0xDEAD_BEEF_FFFF_FFFF;
    emu.regs_aarch64_mut().x[1] = 0xAB;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xFFFF_FABF);
}

#[test]
fn adc_adds_carry_in() {
    let mut emu = emu_with(&[0x9a020020]); // adc x0, x1, x2
    emu.regs_aarch64_mut().x[1] = 5;
    emu.regs_aarch64_mut().x[2] = 7;
    emu.regs_aarch64_mut().nzcv.c = true;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 13);
}

#[test]
fn adcs_32bit_wraps_and_sets_carry() {
    let mut emu = emu_with(&[0x3a020020]); // adcs w0, w1, w2
    emu.regs_aarch64_mut().x[1] = 0xFFFF_FFFF;
    emu.regs_aarch64_mut().x[2] = 0;
    emu.regs_aarch64_mut().nzcv.c = true;
    step_ok(&mut emu);
    let r = emu.regs_aarch64();
    assert_eq!(r.x[0], 0);
    assert!(r.nzcv.z && r.nzcv.c && !r.nzcv.n && !r.nzcv.v);
}

#[test]
fn sbcs_borrows_when_carry_clear_result() {
    let mut emu = emu_with(&[0xfa020020]); // sbcs x0, x1, x2
    emu.regs_aarch64_mut().x[1] = 5;
    emu.regs_aarch64_mut().x[2] = 7;
    emu.regs_aarch64_mut().nzcv.c = true; // no pending borrow
    step_ok(&mut emu);
    let r = emu.regs_aarch64();
    assert_eq!(r.x[0], (-2i64) as u64);
    assert!(r.nzcv.n && !r.nzcv.c && !r.nzcv.z);
}

#[test]
fn ngc_negates_with_borrow() {
    let mut emu = emu_with(&[0xda0103e0, 0xda0103e0]); // ngc x0, x1 (x2)
    emu.regs_aarch64_mut().x[1] = 1;
    emu.regs_aarch64_mut().nzcv.c = true;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], u64::MAX);
    emu.regs_aarch64_mut().nzcv.c = false;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], (-2i64) as u64);
}

#[test]
fn cls_counts_redundant_sign_bits() {
    let cases: [(u64, u64); 4] = [(0, 63), (1, 62), (u64::MAX, 63), (0xFF00 << 48, 7)];
    for (input, expected) in cases {
        let mut emu = emu_with(&[0xdac01420]); // cls x0, x1
        emu.regs_aarch64_mut().x[1] = input;
        step_ok(&mut emu);
        assert_eq!(emu.regs_aarch64().x[0], expected, "cls x of 0x{:x}", input);
    }
    let mut emu = emu_with(&[0x5ac01420]); // cls w0, w1
    emu.regs_aarch64_mut().x[1] = 0xFFFF_FFFF_0000_8000;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 15);
}

#[test]
fn smsubl_uses_signed_low_words() {
    let mut emu = emu_with(&[0x9b228c20]); // smsubl x0, w1, w2, x3
    emu.regs_aarch64_mut().x[1] = 0x1234_5678_FFFF_FFFD; // w1 = -3
    emu.regs_aarch64_mut().x[2] = 4;
    emu.regs_aarch64_mut().x[3] = 100;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 112);
}

#[test]
fn umsubl_uses_unsigned_low_words() {
    let mut emu = emu_with(&[0x9ba28c20]); // umsubl x0, w1, w2, x3
    emu.regs_aarch64_mut().x[1] = 0xFFFF_FFFF;
    emu.regs_aarch64_mut().x[2] = 2;
    emu.regs_aarch64_mut().x[3] = 0x2_0000_0000;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 2);
}

#[test]
fn smull_and_umull_widen() {
    let mut emu = emu_with(&[0x9b227c20, 0x9ba27c20]); // smull x0,w1,w2 ; umull x0,w1,w2
    emu.regs_aarch64_mut().x[1] = (-2i32) as u32 as u64;
    emu.regs_aarch64_mut().x[2] = 3;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], (-6i64) as u64);

    emu.regs_aarch64_mut().x[1] = 0xFFFF_FFFF;
    emu.regs_aarch64_mut().x[2] = 0xFFFF_FFFF;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xFFFF_FFFE_0000_0001);
}

#[test]
fn shifted_w_operand_uses_32bit_asr() {
    let mut emu = emu_with(&[0x0b820c20]); // add w0, w1, w2, asr #3
    emu.regs_aarch64_mut().x[1] = 0;
    emu.regs_aarch64_mut().x[2] = 0x8000_0000;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xF000_0000);
}

#[test]
fn ldaxr_stlxr_round_trip() {
    let mut emu = emu_with(&[0xc85ffc20, 0xc802fc20]); // ldaxr x0,[x1] ; stlxr w2,x0,[x1]
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 0x1122_3344_5566_7788);
    emu.regs_aarch64_mut().x[1] = base;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0x1122_3344_5566_7788);

    emu.regs_aarch64_mut().x[0] = 0xCAFE;
    emu.regs_aarch64_mut().x[2] = 7;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(0xCAFE));
    assert_eq!(
        emu.regs_aarch64().x[2],
        0,
        "store-exclusive must report success"
    );
}

#[test]
fn ldxp_stxp_round_trip() {
    let mut emu = emu_with(&[0xc87f0440, 0xc8230440]); // ldxp x0,x1,[x2] ; stxp w3,x0,x1,[x2]
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 1);
    emu.maps.write_qword(base + 8, 2);
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!((emu.regs_aarch64().x[0], emu.regs_aarch64().x[1]), (1, 2));

    emu.regs_aarch64_mut().x[0] = 3;
    emu.regs_aarch64_mut().x[1] = 4;
    emu.regs_aarch64_mut().x[3] = 9;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(3));
    assert_eq!(emu.maps.read_qword(base + 8), Some(4));
    assert_eq!(emu.regs_aarch64().x[3], 0);
}

#[test]
fn byte_acquire_release_only_touch_one_byte() {
    // ldarb w0,[x1] ; stlrb w0,[x1] ; ldaxrb w3,[x1] ; stlxrb w2,w0,[x1]
    let mut emu = emu_with(&[0x08dffc20, 0x089ffc20, 0x085ffc23, 0x0802fc20]);
    let base = data_map(&mut emu);
    emu.maps.write_word(base, 0x55AB);
    emu.regs_aarch64_mut().x[0] = 0xDEAD_BEEF;
    emu.regs_aarch64_mut().x[1] = base;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xAB);

    emu.regs_aarch64_mut().x[0] = 0x1FF;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_word(base), Some(0x55FF));

    step_ok(&mut emu); // ldaxrb takes the reservation
    assert_eq!(emu.regs_aarch64().x[3], 0xFF);

    emu.regs_aarch64_mut().x[0] = 0x12;
    emu.regs_aarch64_mut().x[2] = 1;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_word(base), Some(0x5512));
    assert_eq!(emu.regs_aarch64().x[2], 0);
}

#[test]
fn prfm_is_a_no_op() {
    let mut emu = emu_with(&[0xf9802000, 0xf89f8000]); // prfm ; prfum
    let pc = emu.regs_aarch64().pc;
    let before = emu.regs_aarch64().x;
    step_ok(&mut emu);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().pc, pc + 8);
    assert_eq!(emu.regs_aarch64().x, before);
}

#[test]
fn ldnp_stnp_have_no_writeback() {
    // ldnp x0,x1,[x2,#16] ; stnp x0,x1,[x2,#16]
    let mut emu = emu_with(&[0xa8410440, 0xa8010440]);
    let base = data_map(&mut emu);
    emu.maps.write_qword(base + 16, 10);
    emu.maps.write_qword(base + 24, 20);
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!((emu.regs_aarch64().x[0], emu.regs_aarch64().x[1]), (10, 20));
    assert_eq!(emu.regs_aarch64().x[2], base);

    emu.regs_aarch64_mut().x[0] = 30;
    emu.regs_aarch64_mut().x[1] = 40;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base + 16), Some(30));
    assert_eq!(emu.maps.read_qword(base + 24), Some(40));
    assert_eq!(emu.regs_aarch64().x[2], base);
}

#[test]
fn ldpsw_sign_extends_both_words() {
    let mut emu = emu_with(&[0x69410440]); // ldpsw x0, x1, [x2, #8]
    let base = data_map(&mut emu);
    emu.maps.write_dword(base + 8, 0xFFFF_FFFE);
    emu.maps.write_dword(base + 12, 5);
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], (-2i64) as u64);
    assert_eq!(emu.regs_aarch64().x[1], 5);
}

#[test]
fn cas_swaps_only_on_match() {
    let mut emu = emu_with(&[0xc8a07c41, 0xc8a07c41]); // cas x0, x1, [x2] (x2)
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 10);
    emu.regs_aarch64_mut().x[0] = 10;
    emu.regs_aarch64_mut().x[1] = 99;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(99));
    assert_eq!(emu.regs_aarch64().x[0], 10);

    emu.regs_aarch64_mut().x[0] = 11;
    emu.regs_aarch64_mut().x[1] = 5;
    step_ok(&mut emu);
    assert_eq!(
        emu.maps.read_qword(base),
        Some(99),
        "mismatch must not store"
    );
    assert_eq!(emu.regs_aarch64().x[0], 99, "Rs receives the old value");
}

#[test]
fn casb_compares_low_byte() {
    let mut emu = emu_with(&[0x08a07c41]); // casb w0, w1, [x2]
    let base = data_map(&mut emu);
    emu.maps.write_word(base, 0x3380);
    emu.regs_aarch64_mut().x[0] = 0x180;
    emu.regs_aarch64_mut().x[1] = 0x7F;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_word(base), Some(0x337F));
    assert_eq!(emu.regs_aarch64().x[0], 0x80);
}

#[test]
fn casp_swaps_register_pair() {
    let mut emu = emu_with(&[0x48207c82]); // casp x0, x1, x2, x3, [x4]
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 1);
    emu.maps.write_qword(base + 8, 2);
    let r = emu.regs_aarch64_mut();
    r.x[0] = 1;
    r.x[1] = 2;
    r.x[2] = 7;
    r.x[3] = 8;
    r.x[4] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(7));
    assert_eq!(emu.maps.read_qword(base + 8), Some(8));
    assert_eq!((emu.regs_aarch64().x[0], emu.regs_aarch64().x[1]), (1, 2));
}

/// Run one `<op> x0, x1, [x2]` atomic with memory `old` and x0 `operand`;
/// returns (new memory qword, x1).
fn run_rmw64(word: u32, old: u64, operand: u64) -> (u64, u64) {
    let mut emu = emu_with(&[word]);
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, old);
    emu.regs_aarch64_mut().x[0] = operand;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    (emu.maps.read_qword(base).unwrap(), emu.regs_aarch64().x[1])
}

#[test]
fn lse_read_modify_write_64() {
    let neg5 = (-5i64) as u64;
    assert_eq!(run_rmw64(0xf8208041, 5, 9), (9, 5), "swp");
    assert_eq!(run_rmw64(0xf8200041, 5, 3), (8, 5), "ldadd");
    assert_eq!(
        run_rmw64(0xf8201041, 0b1111, 0b0101),
        (0b1010, 0b1111),
        "ldclr"
    );
    assert_eq!(
        run_rmw64(0xf8203041, 0b1000, 0b0001),
        (0b1001, 0b1000),
        "ldset"
    );
    assert_eq!(run_rmw64(0xf8202041, 0xFF, 0x0F), (0xF0, 0xFF), "ldeor");
    assert_eq!(run_rmw64(0xf8204041, neg5, 3), (3, neg5), "ldsmax");
    assert_eq!(run_rmw64(0xf8207041, neg5, 3), (3, neg5), "ldumin");
}

#[test]
fn ldaddal_32bit_wraps_without_touching_next_word() {
    let mut emu = emu_with(&[0xb8e00041]); // ldaddal w0, w1, [x2]
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 0xAAAA_AAAA_FFFF_FFFF);
    emu.regs_aarch64_mut().x[0] = 1;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(0xAAAA_AAAA_0000_0000));
    assert_eq!(emu.regs_aarch64().x[1], 0xFFFF_FFFF);
}

#[test]
fn byte_atomics_respect_width_and_sign() {
    let mut emu = emu_with(&[0x38200041]); // ldaddb w0, w1, [x2]
    let base = data_map(&mut emu);
    emu.maps.write_word(base, 0x11FF);
    emu.regs_aarch64_mut().x[0] = 1;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_word(base), Some(0x1100));
    assert_eq!(emu.regs_aarch64().x[1], 0xFF);

    let mut emu = emu_with(&[0x38204041]); // ldsmaxb w0, w1, [x2]
    let base = data_map(&mut emu);
    emu.maps.write_byte(base, 0xFE); // -2
    emu.regs_aarch64_mut().x[0] = 1;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_byte(base), Some(1), "signed max(-2, 1) = 1");
}

#[test]
fn stadd_discards_old_value() {
    let mut emu = emu_with(&[0xf820005f]); // stadd x0, [x2]
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 40);
    emu.regs_aarch64_mut().x[0] = 2;
    emu.regs_aarch64_mut().x[2] = base;
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_qword(base), Some(42));
}

#[test]
fn stxr_without_exclusive_load_fails() {
    let mut emu = emu_with(&[0xc8027c23]); // stxr w2, x3, [x1]
    let base = data_map(&mut emu);
    emu.regs_aarch64_mut().x[1] = base;
    emu.regs_aarch64_mut().x[3] = 0x99;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[2], 1, "no reservation: must fail");
    assert_eq!(emu.maps.read_qword(base), Some(0));
}

#[test]
fn stxr_fails_when_memory_changed_since_ldxr() {
    // ldxr x0,[x1] ; stxr w2,x3,[x1]; another thread's write in between
    let mut emu = emu_with(&[0xc85f7c20, 0xc8027c23]);
    let base = data_map(&mut emu);
    emu.maps.write_qword(base, 5);
    emu.regs_aarch64_mut().x[1] = base;
    emu.regs_aarch64_mut().x[3] = 6;
    step_ok(&mut emu);
    emu.maps.write_qword(base, 7);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[2], 1);
    assert_eq!(emu.maps.read_qword(base), Some(7));
}

#[test]
fn clrex_drops_the_reservation() {
    // ldxr x0,[x1] ; clrex ; stxr w2,x3,[x1]
    let mut emu = emu_with(&[0xc85f7c20, 0xd5033f5f, 0xc8027c23]);
    let base = data_map(&mut emu);
    emu.regs_aarch64_mut().x[1] = base;
    step_ok(&mut emu);
    step_ok(&mut emu);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[2], 1);
}

#[test]
fn register_offset_sxtw_scales_the_index() {
    let mut emu = emu_with(&[0xb82ad928]); // str w8, [x9, w10, sxtw #2]
    let base = data_map(&mut emu);
    emu.regs_aarch64_mut().x[8] = 0xAABB_CCDD;
    emu.regs_aarch64_mut().x[9] = base + 0x100;
    emu.regs_aarch64_mut().x[10] = 0xFFFF_FFFE; // -2 as w10
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_dword(base + 0x100 - 8), Some(0xAABB_CCDD));
}

#[test]
fn add_extended_register_shifts_after_extending() {
    let mut emu = emu_with(&[0x8b22c820]); // add x0, x1, w2, sxtw #2
    emu.regs_aarch64_mut().x[1] = 100;
    emu.regs_aarch64_mut().x[2] = 0xFFFF_FFFF; // -1 as w2
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 96);
}
