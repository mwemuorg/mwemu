//! Regression tests for x86 semantics found by diffing libmwemu against a real
//! CPU running under Rosetta (`scripts/rosetta-difftest/`). Each test is a
//! self-contained stub with the value the silicon produces.

use crate::maps::mem64::Permission;
use crate::*;

const BUF: u64 = 0x3000_0000;

fn run_code(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    emu.run(None).unwrap();
    emu
}

/// Code plus a read-write data buffer at `BUF`, not started yet.
fn emu_with_buffer(code: &[u8], data: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    emu.maps
        .create_map("buf", BUF, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_bytes(BUF, data);
    emu
}

/// `imul rax, rdx` with -1 * 1: the product fits, so CF and OF stay clear.
#[test]
fn imul_two_operand_negative_product_that_fits_clears_cf_of() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -1
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 1
        0x48, 0x0f, 0xaf, 0xc2, // imul rax, rdx
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().rax, u64::MAX);
    assert!(!emu.flags_snapshot().f_cf, "-1 * 1 fits: CF must be clear");
    assert!(!emu.flags_snapshot().f_of, "-1 * 1 fits: OF must be clear");
}

/// `imul eax, eax, 2` on 0x6277f134: the product is 0xc4efe268, which does not
/// fit a signed 32-bit register, so CF=OF=1 even though it fits unsigned.
#[test]
fn imul_three_operand_signed_overflow_sets_cf_of() {
    let code = [
        0xb8, 0x34, 0xf1, 0x77, 0x62, // mov eax, 0x6277f134
        0x6b, 0xc0, 0x02, // imul eax, eax, 2
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().get_eax(), 0xc4ef_e268);
    assert!(emu.flags_snapshot().f_cf, "positive overflow must set CF");
    assert!(emu.flags_snapshot().f_of, "positive overflow must set OF");
}

/// One-operand `imul r/m16` multiplies signed words: -1 * 0x1014 = -4116.
#[test]
fn imul_one_operand_16bit_is_signed() {
    let code = [
        0x66, 0xb8, 0xff, 0xff, // mov ax, 0xffff
        0x66, 0xb9, 0x14, 0x10, // mov cx, 0x1014
        0x66, 0xf7, 0xe9, // imul cx
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().get_dx(),
        0xffff,
        "DX must hold the sign extension"
    );
    assert_eq!(emu.regs().get_ax(), 0xefec);
    assert!(!emu.flags_snapshot().f_cf, "-4116 fits in AX: CF clear");
    assert!(!emu.flags_snapshot().f_of, "-4116 fits in AX: OF clear");
}

/// `div rcx` with RDX:RAX = 2^64 and RCX = 1: the quotient does not fit RAX, so
/// the CPU raises #DE before touching any register and the code after the
/// division never runs.
#[test]
fn div_quotient_overflow_raises_de() {
    let code = [
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 1
        0x48, 0x31, 0xc0, // xor rax, rax
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // mov rcx, 1
        0x48, 0x31, 0xdb, // xor rbx, rbx
        0x48, 0xf7, 0xf1, // div rcx
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // mov rbx, 1
        0xf4, // hlt
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    let _ = emu.run(None);
    assert_eq!(emu.regs().rax, 0, "RAX must not be written on #DE");
    assert_eq!(emu.regs().rdx, 1, "RDX must not be written on #DE");
    assert_eq!(emu.regs().rbx, 0, "execution must stop at the faulting div");
}

/// `div ecx` with EDX:EAX = 2^32 and ECX = 1 raises #DE as well.
#[test]
fn div32_quotient_overflow_raises_de() {
    let code = [
        0xba, 0x01, 0x00, 0x00, 0x00, // mov edx, 1
        0x31, 0xc0, // xor eax, eax
        0xb9, 0x01, 0x00, 0x00, 0x00, // mov ecx, 1
        0x31, 0xdb, // xor ebx, ebx
        0xf7, 0xf1, // div ecx
        0xbb, 0x01, 0x00, 0x00, 0x00, // mov ebx, 1
        0xf4, // hlt
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    let _ = emu.run(None);
    assert_eq!(emu.regs().get_edx(), 1);
    assert_eq!(
        emu.regs().get_ebx(),
        0,
        "execution must stop at the faulting div"
    );
}

/// In 64-bit mode `loopne` without an address-size prefix counts the full RCX:
/// 0x1_0000_0005 - 1 keeps its upper half (the old code picked ECX from the
/// target address and zeroed it).
#[test]
fn loopne_counts_full_rcx_in_64bit_mode() {
    let code = [
        0x48, 0xb9, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // mov rcx, 0x100000005
        0x39, 0xc0, // cmp eax, eax  -> ZF=1, so loopne falls through
        0xe0, 0x00, // loopne +0
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().rcx, 0x1_0000_0004);
}

/// With the 0x67 prefix `loopne` counts ECX and zero-extends the write to RCX.
#[test]
fn loopne_with_address_size_prefix_counts_ecx() {
    let code = [
        0x48, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // mov rcx, 0x100000001
        0x39, 0xc0, // cmp eax, eax
        0x67, 0xe0, 0x00, // loopne +0 (ecx)
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().rcx,
        0,
        "ECX hit zero and the upper half is cleared"
    );
}

/// `jecxz` tests ECX, not CX: ECX = 0x80000000 must not jump.
#[test]
fn jecxz_tests_ecx_not_cx() {
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x80, // mov ecx, 0x80000000
        0x31, 0xc0, // xor eax, eax
        0x67, 0xe3, 0x02, // jecxz +2
        0xb0, 0x01, // mov al, 1
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().get_al(),
        1,
        "jecxz must not be taken with ECX != 0"
    );
}

fn repne_scas_test(code: &[u8], data: &[u8], acc: u64, elem: u64) {
    let mut emu = emu_with_buffer(code, data);
    emu.regs_mut().rax = acc;
    emu.regs_mut().rdi = BUF;
    emu.regs_mut().rcx = 4;
    emu.run(None).unwrap();
    assert_eq!(
        emu.regs().rcx,
        1,
        "the third element matches the accumulator"
    );
    assert_eq!(emu.regs().rdi, BUF + 3 * elem);
    assert!(emu.flags_snapshot().f_zf);
}

/// `repne scasw/scasd/scasq` compare the accumulator with the string element
/// (the old handlers compared the accumulator with itself and always matched).
#[test]
fn repne_scasw_compares_ax_with_memory() {
    let data = [1u16, 2, 3, 4]
        .iter()
        .flat_map(|w| w.to_le_bytes())
        .collect::<Vec<_>>();
    repne_scas_test(&[0xfc, 0xf2, 0x66, 0xaf, 0xf4], &data, 3, 2);
}

#[test]
fn repne_scasd_compares_eax_with_memory() {
    let data = [1u32, 2, 3, 4]
        .iter()
        .flat_map(|d| d.to_le_bytes())
        .collect::<Vec<_>>();
    repne_scas_test(&[0xfc, 0xf2, 0xaf, 0xf4], &data, 3, 4);
}

#[test]
fn repne_scasq_compares_rax_with_memory() {
    let data = [1u64, 2, 3, 4]
        .iter()
        .flat_map(|q| q.to_le_bytes())
        .collect::<Vec<_>>();
    repne_scas_test(&[0xfc, 0xf2, 0x48, 0xaf, 0xf4], &data, 3, 8);
}

/// `bts dword ptr [rsi], eax` with EAX = 40 sets bit 8 of the *second* dword:
/// a register bit offset on a memory base addresses neighbouring memory.
#[test]
fn bts_memory_register_offset_reaches_next_element() {
    let code = [
        0xb8, 0x28, 0x00, 0x00, 0x00, // mov eax, 40
        0x0f, 0xab, 0x06, // bts dword ptr [rsi], eax
        0xf4, // hlt
    ];
    let mut emu = emu_with_buffer(&code, &[0u8; 16]);
    emu.regs_mut().rsi = BUF;
    emu.run(None).unwrap();
    assert_eq!(emu.maps.read_dword(BUF), Some(0), "first dword untouched");
    assert_eq!(emu.maps.read_dword(BUF + 4), Some(1 << 8));
    assert!(!emu.flags_snapshot().f_cf, "the bit was clear before");
}

/// A negative register offset addresses the element *before* the base:
/// `bt qword ptr [rsi], rax` with RAX = -1 reads bit 63 of [rsi - 8].
#[test]
fn bt_memory_negative_register_offset_reads_previous_element() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -1
        0x48, 0x0f, 0xa3, 0x06, // bt qword ptr [rsi], rax
        0xf4, // hlt
    ];
    let mut data = [0u8; 16];
    data[7] = 0x80; // bit 63 of the first qword
    let mut emu = emu_with_buffer(&code, &data);
    emu.regs_mut().rsi = BUF + 8;
    emu.run(None).unwrap();
    assert!(
        emu.flags_snapshot().f_cf,
        "bit 63 of the previous qword is set"
    );
}

/// Self-contained shellcode: the code map ends right after `repne scasb`, so
/// the decoded block holds nothing past it and the run loop used to resume the
/// REP with a zero instruction size, re-running it until RCX hit zero.
#[test]
fn rep_terminates_when_the_decoded_block_ends_on_it() {
    let code = [
        0xfc, // cld
        0x48, 0x8d, 0x3d, 0x0a, 0x00, 0x00, 0x00, // lea rdi, [rip + 10]  -> data
        0x31, 0xc0, // xor eax, eax
        0xb9, 0x08, 0x00, 0x00, 0x00, // mov ecx, 8
        0xf2, 0xae, // repne scasb
        0xf4, // hlt
        b'a', 0, b'c', b'd', b'e', b'f', b'g', b'h', // data: NUL at index 1
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    let base = emu.regs().rip;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 6, "stop after the second element");
    assert_eq!(emu.regs().rdi, base + 18 + 2);
    assert!(emu.flags_snapshot().f_zf);
}

/// `step()` must execute a REP string instruction completely, like the CPU
/// does for a single instruction, instead of one element with RCX untouched.
#[test]
fn step_runs_the_whole_rep_instruction() {
    let code = [0xf2, 0xae, 0xf4]; // repne scasb ; hlt
    let mut emu = emu_with_buffer(&code, b"ab\0defgh");
    let base = emu.regs().rip;
    emu.regs_mut().rax = 0;
    emu.regs_mut().rdi = BUF;
    emu.regs_mut().rcx = 8;
    assert!(emu.step());
    assert_eq!(emu.regs().rcx, 5, "stopped at the NUL in position 2");
    assert_eq!(emu.regs().rdi, BUF + 3);
    assert_eq!(emu.regs().rip, base + 2, "rip is past the instruction");
}

/// `xlatb` loads AL from the byte table at RBX indexed by AL.
#[test]
fn xlatb_translates_al_through_the_rbx_table() {
    let code = [
        0xb0, 0x05, // mov al, 5
        0xd7, // xlatb
        0xf4, // hlt
    ];
    let table: Vec<u8> = (0u8..16).map(|i| i * 2).collect();
    let mut emu = emu_with_buffer(&code, &table);
    emu.regs_mut().rbx = BUF;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().get_al(), 10);
}

/// 16-bit `shrd`/`shld` with a count above the operand size are undefined but
/// must not abort the emulator (the shift amount used to underflow in debug).
#[test]
fn shrd_shld_16bit_count_above_size_does_not_panic() {
    let code = [
        0x66, 0xb8, 0x34, 0x12, // mov ax, 0x1234
        0x66, 0xbb, 0x78, 0x56, // mov bx, 0x5678
        0x66, 0x0f, 0xac, 0xd8, 0x11, // shrd ax, bx, 17
        0x66, 0x0f, 0xa4, 0xd8, 0x11, // shld ax, bx, 17
        0xf4, // hlt
    ];
    run_code(&code);
}

/// The one-count shift helpers must agree with the two-operand ones.
#[test]
fn one_count_shift_helpers_match_two_operand_forms() {
    let mut emu = emu64();
    assert_eq!(emu.flags_mut().shl1p32(0x8000_0000), 0);
    assert!(emu.flags_mut().f_cf, "shl eax,1 of 0x80000000 carries out");
    assert!(emu.flags_mut().f_of, "sign changed with CF set: OF=1");
    assert_eq!(emu.flags_mut().shl1p8(0x40), 0x80);
    assert!(!emu.flags_mut().f_cf);
    assert!(emu.flags_mut().f_of, "MSB flipped without a carry: OF=1");
    assert_eq!(
        emu.flags_mut().sar1p8(0x80),
        0xc0,
        "sar fills with the sign"
    );
    assert_eq!(emu.flags_mut().shr1p16(0x8001), 0x4000);
    assert!(emu.flags_mut().f_cf);
}
