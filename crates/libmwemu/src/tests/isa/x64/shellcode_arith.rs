//! Hand-crafted shellcode tests: integer arithmetic, flags and width handling.

use super::shellcode_helpers::run;
const ADD64_OVERFLOW: &[u8] = &[
    0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x7f, // movabs rax, 0x7fffffffffffffff
    0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // mov rbx, 0x1
    0x48, 0x01, 0xd8, // add rax, rbx
    0xf4, // hlt
];

const ADD32_CARRY_ZERO: &[u8] = &[
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -0x1
    0xbb, 0x01, 0x00, 0x00, 0x00, // mov ebx, 0x1
    0x01, 0xd8, // add eax, ebx
    0xf4, // hlt
];

const ADC_CHAIN: &[u8] = &[
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -0x1
    0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // mov rbx, 0x5
    0x48, 0x83, 0xc0, 0x01, // add rax, 0x1
    0x48, 0x83, 0xd3, 0x00, // adc rbx, 0x0
    0xf4, // hlt
];

const SUB_SBB_BROADCAST: &[u8] = &[
    0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // mov rax, 0x5
    0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // mov rbx, 0x7
    0x48, 0xc7, 0xc2, 0x34, 0x12, 0x00, 0x00, // mov rdx, 0x1234
    0x48, 0x29, 0xd8, // sub rax, rbx
    0x48, 0x19, 0xd2, // sbb rdx, rdx
    0xf4, // hlt
];

const CMP_SIGNED_OVERFLOW: &[u8] = &[
    0xb8, 0x00, 0x00, 0x00, 0x80, // mov eax, 0x80000000
    0x83, 0xf8, 0x01, // cmp eax, 0x1
    0xf4, // hlt
];

const INC_KEEPS_CF: &[u8] = &[
    0xf9, // stc
    0xb8, 0xff, 0xff, 0xff, 0x7f, // mov eax, 0x7fffffff
    0xff, 0xc0, // inc eax
    0xf4, // hlt
];

const NEG_CASES: &[u8] = &[
    0x31, 0xc0, // xor eax, eax
    0x48, 0xf7, 0xd8, // neg rax
    0x0f, 0x94, 0xc3, // sete bl
    0x0f, 0x92, 0xc7, // setb bh
    0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 0x1
    0x48, 0xf7, 0xd8, // neg rax
    0x0f, 0x92, 0xc1, // setb cl
    0xba, 0x00, 0x00, 0x00, 0x80, // mov edx, 0x80000000
    0xf7, 0xda, // neg edx
    0x0f, 0x90, 0xc2, // seto dl
    0xf4, // hlt
];

const MUL64: &[u8] = &[
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -0x1
    0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // mov rbx, 0x2
    0x48, 0xf7, 0xe3, // mul rbx
    0xf4, // hlt
];

const IMUL_ONE_OPERAND: &[u8] = &[
    0x48, 0xc7, 0xc0, 0xfd, 0xff, 0xff, 0xff, // mov rax, -0x3
    0x48, 0xc7, 0xc3, 0x04, 0x00, 0x00, 0x00, // mov rbx, 0x4
    0x48, 0xf7, 0xeb, // imul rbx
    0xf4, // hlt
];

const IMUL_THREE_OPERAND_EDGE: &[u8] = &[
    0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x40, // movabs rbx, 0x4000000000000000
    0x48, 0x6b, 0xc3, 0xfe, // imul rax, rbx, -0x2
    0x0f, 0x92, 0xc1, // setb cl
    0x48, 0x6b, 0xd3, 0x02, // imul rdx, rbx, 0x2
    0x0f, 0x92, 0xc5, // setb ch
    0xf4, // hlt
];

const DIV128: &[u8] = &[
    0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 0x1
    0x31, 0xc0, // xor eax, eax
    0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // mov rbx, 0x3
    0x48, 0xf7, 0xf3, // div rbx
    0xf4, // hlt
];

const IDIV_NEGATIVE: &[u8] = &[
    0xba, 0xff, 0xff, 0xff, 0xff, // mov edx, 0xffffffff
    0xb8, 0xf9, 0xff, 0xff, 0xff, // mov eax, 0xfffffff9
    0xb9, 0x02, 0x00, 0x00, 0x00, // mov ecx, 0x2
    0xf7, 0xf9, // idiv ecx
    0xf4, // hlt
];

const SIGN_EXTEND_CONVERSIONS: &[u8] = &[
    0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, // movabs rax, -0x8000000000000000
    0x48, 0x99, // cqo
    0x48, 0x89, 0xd3, // mov rbx, rdx
    0xb8, 0xff, 0xff, 0xff, 0x7f, // mov eax, 0x7fffffff
    0x99, // cdq
    0x48, 0x89, 0xd1, // mov rcx, rdx
    0xb0, 0x80, // mov al, -0x80
    0x66, 0x98, // cbw
    0x48, 0x89, 0xc2, // mov rdx, rax
    0x98, // cwde
    0x48, 0x89, 0xc6, // mov rsi, rax
    0x48, 0x98, // cdqe
    0xf4, // hlt
];

const MOVSX_MOVZX: &[u8] = &[
    0x48, 0xb8, 0xf0, 0xff, 0xff, 0xff, 0x44, 0x33, 0x22,
    0x11, // movabs rax, 0x11223344fffffff0
    0x49, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov r8, -0x1
    0x48, 0x63, 0xd8, // movsxd rbx, eax
    0x48, 0x0f, 0xbe, 0xc8, // movsx rcx, al
    0x0f, 0xb7, 0xd0, // movzx edx, ax
    0x66, 0x44, 0x0f, 0xbe, 0xc0, // movsx r8w, al
    0xf4, // hlt
];

const PARTIAL_REGISTERS: &[u8] = &[
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -0x1
    0xb4, 0x12, // mov ah, 0x12
    0x48, 0x89, 0xc3, // mov rbx, rax
    0x66, 0xb8, 0x78, 0x56, // mov ax, 0x5678
    0x48, 0x89, 0xc1, // mov rcx, rax
    0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 0x1
    0x48, 0x89, 0xc2, // mov rdx, rax
    0x86, 0xdf, // xchg bl, bh
    0xf4, // hlt
];

#[test]
fn add_64bit_signed_overflow() {
    let e = run(ADD64_OVERFLOW, |_| {});
    assert_eq!(e.regs().rax, 0x8000_0000_0000_0000);
    let f = e.flags_snapshot();
    assert!(f.f_of && f.f_sf, "max + 1 overflows into the sign bit");
    assert!(!f.f_cf && !f.f_zf, "no unsigned carry, result not zero");
}

#[test]
fn add_32bit_carry_and_zero_extension() {
    let e = run(ADD32_CARRY_ZERO, |_| {});
    assert_eq!(e.regs().rax, 0, "a 32-bit result clears the upper half");
    let f = e.flags_snapshot();
    assert!(f.f_cf && f.f_zf && !f.f_of);
}

#[test]
fn adc_propagates_the_carry_into_the_next_register() {
    let e = run(ADC_CHAIN, |_| {});
    assert_eq!(e.regs().rax, 0);
    assert_eq!(e.regs().rbx, 6, "adc consumed the carry of the add");
    assert!(!e.flags_snapshot().f_cf);
}

#[test]
fn sub_borrow_and_sbb_sign_broadcast() {
    let e = run(SUB_SBB_BROADCAST, |_| {});
    assert_eq!(e.regs().rax, 0xffff_ffff_ffff_fffe);
    assert_eq!(e.regs().rdx, u64::MAX, "sbb reg, reg broadcasts the borrow");
    let f = e.flags_snapshot();
    assert!(f.f_cf && f.f_sf);
}

#[test]
fn cmp_signed_overflow_without_unsigned_borrow() {
    let e = run(CMP_SIGNED_OVERFLOW, |_| {});
    assert_eq!(e.regs().get_eax(), 0x8000_0000, "cmp does not write");
    let f = e.flags_snapshot();
    assert!(f.f_of && !f.f_cf && !f.f_sf);
}

#[test]
fn inc_keeps_the_carry_and_sets_overflow() {
    let e = run(INC_KEEPS_CF, |_| {});
    assert_eq!(e.regs().rax, 0x8000_0000);
    let f = e.flags_snapshot();
    assert!(f.f_cf, "inc never touches CF");
    assert!(f.f_of && f.f_sf);
}

#[test]
fn neg_of_zero_one_and_minimum() {
    let e = run(NEG_CASES, |_| {});
    assert_eq!(e.regs().rbx & 0xffff, 0x0001, "neg 0: ZF=1, CF=0");
    assert_eq!(e.regs().rcx & 0xff, 1, "neg 1: CF=1");
    assert_eq!(
        e.regs().rdx,
        0x8000_0001,
        "neg INT_MIN: OF=1, value unchanged"
    );
    assert_eq!(e.regs().rax, u64::MAX);
}

#[test]
fn mul_64_produces_a_128_bit_product() {
    let e = run(MUL64, |_| {});
    assert_eq!(e.regs().rax, 0xffff_ffff_ffff_fffe);
    assert_eq!(e.regs().rdx, 1);
    let f = e.flags_snapshot();
    assert!(f.f_cf && f.f_of, "high half is non-zero");
}

#[test]
fn imul_one_operand_is_signed() {
    let e = run(IMUL_ONE_OPERAND, |_| {});
    assert_eq!(e.regs().rax as i64, -12);
    assert_eq!(e.regs().rdx, u64::MAX, "sign extension of the product");
    let f = e.flags_snapshot();
    assert!(!f.f_cf && !f.f_of, "-12 fits in RAX");
}

#[test]
fn imul_three_operand_overflow_boundary() {
    let e = run(IMUL_THREE_OPERAND_EDGE, |_| {});
    assert_eq!(e.regs().rax, 0x8000_0000_0000_0000, "-2 * 2^62 = i64::MIN");
    assert_eq!(e.regs().rdx, 0x8000_0000_0000_0000, "2 * 2^62 wraps");
    assert_eq!(e.regs().rcx, 0x0100, "cl: exact fit, ch: overflow");
}

#[test]
fn div_128_by_64() {
    let e = run(DIV128, |_| {});
    assert_eq!(e.regs().rax, 0x5555_5555_5555_5555, "2^64 / 3");
    assert_eq!(e.regs().rdx, 1, "remainder");
}

#[test]
fn idiv_truncates_toward_zero() {
    let e = run(IDIV_NEGATIVE, |_| {});
    assert_eq!(e.regs().rax, 0xffff_fffd, "-7 / 2 = -3, zero-extended");
    assert_eq!(e.regs().rdx, 0xffff_ffff, "remainder -1");
}

#[test]
fn sign_extension_conversions() {
    let e = run(SIGN_EXTEND_CONVERSIONS, |_| {});
    assert_eq!(e.regs().rbx, u64::MAX, "cqo of a negative rax");
    assert_eq!(e.regs().rcx, 0, "cdq of a positive eax");
    assert_eq!(
        e.regs().rdx,
        0x7fff_ff80,
        "cbw: ax = 0xff80, upper eax kept"
    );
    assert_eq!(e.regs().rsi, 0xffff_ff80, "cwde zero-extends into rax");
    assert_eq!(e.regs().rax, 0xffff_ffff_ffff_ff80, "cdqe");
}

#[test]
fn movsx_movzx_at_every_width() {
    let e = run(MOVSX_MOVZX, |_| {});
    assert_eq!(e.regs().rbx, 0xffff_ffff_ffff_fff0, "movsxd");
    assert_eq!(e.regs().rcx, 0xffff_ffff_ffff_fff0, "movsx r64, r8");
    assert_eq!(e.regs().rdx, 0xfff0, "movzx r32, r16 clears the rest");
    assert_eq!(
        e.regs().r8,
        0xffff_ffff_ffff_fff0,
        "movsx r16, r8 keeps the upper 48 bits"
    );
}

#[test]
fn partial_register_writes() {
    let e = run(PARTIAL_REGISTERS, |_| {});
    assert_eq!(e.regs().rax, 1, "a 32-bit write zero-extends");
    assert_eq!(
        e.regs().rbx,
        0xffff_ffff_ffff_ff12,
        "ah write merged, then bl/bh swapped"
    );
    assert_eq!(e.regs().rcx, 0xffff_ffff_ffff_5678, "a 16-bit write merges");
    assert_eq!(e.regs().rdx, 1);
}
