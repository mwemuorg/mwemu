//! Hand-crafted 32-bit shellcode tests: no sample binaries, each test runs a
//! small program to `hlt` under `emu32()` and checks the architectural state.

use crate::maps::mem64::Permission;
use crate::*;

const BUF: u64 = 0x3000_0000;

fn prepare(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = emu32();
    emu.os = crate::arch::OperatingSystem::Linux; // no Windows maps needed
    emu.load_code_bytes(code);
    setup(&mut emu);
    emu
}

fn run(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = prepare(code, setup);
    emu.run(None).unwrap();
    emu
}

const PUSHA_POPA: &[u8] = &[
    0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 0x1
    0xbb, 0x02, 0x00, 0x00, 0x00, // mov ebx, 0x2
    0xb9, 0x03, 0x00, 0x00, 0x00, // mov ecx, 0x3
    0xba, 0x04, 0x00, 0x00, 0x00, // mov edx, 0x4
    0xbe, 0x05, 0x00, 0x00, 0x00, // mov esi, 0x5
    0xbf, 0x06, 0x00, 0x00, 0x00, // mov edi, 0x6
    0xbd, 0x07, 0x00, 0x00, 0x00, // mov ebp, 0x7
    0x60, // pushal
    0x31, 0xc0, // xor eax, eax
    0x31, 0xdb, // xor ebx, ebx
    0x31, 0xc9, // xor ecx, ecx
    0x31, 0xd2, // xor edx, edx
    0x31, 0xf6, // xor esi, esi
    0x31, 0xff, // xor edi, edi
    0x31, 0xed, // xor ebp, ebp
    0x61, // popal
    0xf4, // hlt
];
const LOOP32_SUM: &[u8] = &[
    0x31, 0xc0, // xor eax, eax
    0xb9, 0x64, 0x00, 0x00, 0x00, // mov ecx, 0x64
    0x01, 0xc8, // add eax, ecx
    0xe2, 0xfc, // loop 0x7
    0xf4, // hlt
];
const JECXZ32_TESTS_ECX: &[u8] = &[
    0xb9, 0x00, 0x00, 0x01, 0x00, // mov ecx, 0x10000
    0x31, 0xc0, // xor eax, eax
    0xe3, 0x05, // jecxz 0xe
    0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 0x1
    0xf4, // hlt
];
const CALL_RET32: &[u8] = &[
    0xe8, 0x06, 0x00, 0x00, 0x00, // call 0xb
    0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 0x1
    0xf4, // hlt
    0xbb, 0x02, 0x00, 0x00, 0x00, // mov ebx, 0x2
    0xc3, // ret
];
const BCD_ARITHMETIC: &[u8] = &[
    0x66, 0xb8, 0x0f, 0x00, // mov ax, 0xf
    0x37, // aaa
    0x66, 0x89, 0xc3, // mov bx, ax
    0x66, 0xb8, 0x07, 0x01, // mov ax, 0x107
    0xd5, 0x0a, // aad
    0x66, 0x89, 0xc1, // mov cx, ax
    0x66, 0xb8, 0x2d, 0x00, // mov ax, 0x2d
    0xd4, 0x0a, // aam
    0x66, 0x89, 0xc2, // mov dx, ax
    0xb0, 0x9a, // mov al, -0x66
    0x2f, // das
    0x66, 0x89, 0xc6, // mov si, ax
    0xb0, 0x9a, // mov al, -0x66
    0x27, // daa
    0x66, 0x89, 0xc7, // mov di, ax
    0xf4, // hlt
];
const STRING32: &[u8] = &[
    0xfc, // cld
    0xb9, 0x05, 0x00, 0x00, 0x00, // mov ecx, 0x5
    0xf3, 0xa4, // rep  movsb byte ptr es:[edi], byte ptr [esi]
    0x31, 0xc0, // xor eax, eax
    0xb9, 0xff, 0xff, 0xff, 0xff, // mov ecx, 0xffffffff
    0x89, 0xdf, // mov edi, ebx
    0xf2, 0xae, // repne  scasb al, byte ptr es:[edi]
    0xf7, 0xd1, // not ecx
    0x49, // dec ecx
    0xf4, // hlt
];
const OPSIZE16: &[u8] = &[
    0xb8, 0x78, 0x56, 0x34, 0x12, // mov eax, 0x12345678
    0x66, 0xb8, 0xff, 0xff, // mov ax, 0xffff
    0x66, 0x40, // inc ax
    0xbb, 0x01, 0xef, 0xcd, 0xab, // mov ebx, 0xabcdef01
    0x66, 0x93, // xchg ax, bx
    0xf4, // hlt
];
const ENTER_LEAVE: &[u8] = &[
    0x89, 0xe0, // mov eax, esp
    0xc8, 0x10, 0x00, 0x00, // enter 0x10, 0x0
    0x89, 0xe3, // mov ebx, esp
    0x89, 0xe9, // mov ecx, ebp
    0xc9, // leave
    0x29, 0xe0, // sub eax, esp
    0xf4, // hlt
];
const SHIFTS32: &[u8] = &[
    0xb8, 0x01, 0x00, 0x00, 0x80, // mov eax, 0x80000001
    0xb1, 0x01, // mov cl, 0x1
    0xd3, 0xe8, // shr eax, cl
    0x0f, 0x92, 0xc3, // setb bl
    0xba, 0xff, 0xff, 0xff, 0x7f, // mov edx, 0x7fffffff
    0xd1, 0xe2, // shl edx
    0x0f, 0x90, 0xc7, // seto bh
    0xd1, 0xda, // rcr edx
    0xf4, // hlt
];

#[test]
fn pusha_popa_round_trip() {
    let mut e = prepare(PUSHA_POPA, |_| {});
    let esp = e.regs().get_esp();
    e.run(None).unwrap();
    let r = e.regs();
    assert_eq!(
        (r.get_eax(), r.get_ebx(), r.get_ecx(), r.get_edx()),
        (1, 2, 3, 4)
    );
    assert_eq!((r.get_esi(), r.get_edi(), r.get_ebp()), (5, 6, 7));
    assert_eq!(r.get_esp(), esp, "popa skips the saved esp");
}

#[test]
fn loop_counts_ecx() {
    let e = run(LOOP32_SUM, |_| {});
    assert_eq!(e.regs().get_eax(), 5050);
}

#[test]
fn jecxz_tests_the_whole_ecx() {
    let e = run(JECXZ32_TESTS_ECX, |_| {});
    assert_eq!(
        e.regs().get_eax(),
        1,
        "ecx = 0x10000 has cx = 0 but is not zero"
    );
}

#[test]
fn call_and_ret() {
    let mut e = prepare(CALL_RET32, |_| {});
    let esp = e.regs().get_esp();
    e.run(None).unwrap();
    assert_eq!((e.regs().get_eax(), e.regs().get_ebx()), (1, 2));
    assert_eq!(e.regs().get_esp(), esp);
}

#[test]
fn bcd_adjust_instructions() {
    let e = run(BCD_ARITHMETIC, |_| {});
    assert_eq!(e.regs().get_bx(), 0x0105, "aaa of 0x0f: al = 5, ah = 1");
    assert_eq!(e.regs().get_cx(), 0x0011, "aad of 01:07 = 17");
    assert_eq!(e.regs().get_dx(), 0x0405, "aam of 45 = 4:5");
    assert_eq!(e.regs().get_si(), 0x0434, "das of 0x9a");
    assert_eq!(
        e.regs().get_di(),
        0x0400,
        "daa of 0x9a with CF set wraps to 0"
    );
    assert!(e.flags_snapshot().f_cf, "daa carried");
}

#[test]
fn string_instructions_with_32_bit_pointers() {
    let e = run(STRING32, |emu| {
        emu.maps
            .create_map("buf", BUF, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_bytes(BUF, b"hello\0");
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 32;
        emu.regs_mut().rbx = BUF;
    });
    assert_eq!(e.maps.read_bytes(BUF + 32, 5), b"hello", "rep movsb");
    assert_eq!(e.regs().get_ecx(), 5, "repne scasb strlen");
    assert_eq!(e.regs().get_edi(), BUF + 6, "edi stops after the NUL");
}

#[test]
fn sixteen_bit_operand_size() {
    let e = run(OPSIZE16, |_| {});
    assert_eq!(
        e.regs().get_eax(),
        0x1234_ef01,
        "inc ax wrapped to 0, then xchg with bx"
    );
    assert_eq!(e.regs().get_ebx(), 0xabcd_0000);
    assert!(e.flags_snapshot().f_zf, "inc ax reached zero");
}

#[test]
fn enter_and_leave_restore_the_frame() {
    let e = run(ENTER_LEAVE, |_| {});
    assert_eq!(e.regs().get_eax(), 0, "esp restored by leave");
    assert_eq!(
        e.regs().get_ecx() - e.regs().get_ebx(),
        16,
        "enter reserved 16 bytes"
    );
}

#[test]
fn shifts_and_rotate_through_carry() {
    let e = run(SHIFTS32, |_| {});
    assert_eq!(e.regs().get_eax(), 0x4000_0000);
    assert_eq!(e.regs().get_ebx(), 0x0101, "shr set CF, shl set OF");
    assert_eq!(
        e.regs().get_edx(),
        0x7fff_ffff,
        "rcr pulled the clear CF into bit 31"
    );
}
