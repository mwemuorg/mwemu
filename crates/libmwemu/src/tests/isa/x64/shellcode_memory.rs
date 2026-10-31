//! Hand-crafted shellcode tests: memory operands, the stack, atomics, string
//! instructions and small algorithms over a data buffer.

use super::shellcode_helpers::{BUF, data, run_with_data};
const ADDRESSING_MODES: &[u8] = &[
    0x48, 0x89, 0xf3, // mov rbx, rsi
    0x48, 0xc7, 0x06, 0x11, 0x00, 0x00, 0x00, // mov qword ptr [rsi], 0x11
    0xc7, 0x46, 0x08, 0x22, 0x00, 0x00, 0x00, // mov dword ptr [rsi + 0x8], 0x22
    0x66, 0xc7, 0x04, 0x4e, 0x33, 0x33, // mov word ptr [rsi + 2*rcx], 0x3333
    0xc6, 0x44, 0x0b, 0x01, 0x44, // mov byte ptr [rbx + rcx + 0x1], 0x44
    0x8b, 0x06, // mov eax, dword ptr [rsi]
    0x48, 0x8b, 0x56, 0x04, // mov rdx, qword ptr [rsi + 0x4]
    0x0f, 0xb7, 0x7e, 0x08, // movzx edi, word ptr [rsi + 0x8]
    0x48, 0x81, 0x06, 0x00, 0x01, 0x00, 0x00, // add qword ptr [rsi], 0x100
    0xfe, 0x46, 0x09, // inc byte ptr [rsi + 0x9]
    0xf4, // hlt
];

const PUSH_POP_MEMORY: &[u8] = &[
    0xff, 0x36, // push qword ptr [rsi]
    0x8f, 0x46, 0x08, // pop qword ptr [rsi + 0x8]
    0x56, // push rsi
    0x58, // pop rax
    0xf4, // hlt
];

const CAS_LOOP: &[u8] = &[
    0xb9, 0x05, 0x00, 0x00, 0x00, // mov ecx, 0x5
    0x48, 0x8b, 0x06, // mov rax, qword ptr [rsi]
    0x48, 0x8d, 0x50, 0x07, // lea rdx, [rax + 0x7]
    0x48, 0x0f, 0xb1, 0x16, // cmpxchg qword ptr [rsi], rdx
    0x75, 0xf3, // jne 0x5
    0xff, 0xc9, // dec ecx
    0x75, 0xef, // jne 0x5
    0xf4, // hlt
];

const REP_MOVSB_FORWARD: &[u8] = &[
    0xfc, // cld
    0xb9, 0x10, 0x00, 0x00, 0x00, // mov ecx, 0x10
    0xf3, 0xa4, // rep  movsb byte ptr es:[rdi], byte ptr [rsi]
    0xf4, // hlt
];

const REP_MOVSB_OVERLAP_BACKWARD: &[u8] = &[
    0xfd, // std
    0xb9, 0x10, 0x00, 0x00, 0x00, // mov ecx, 0x10
    0xf3, 0xa4, // rep  movsb byte ptr es:[rdi], byte ptr [rsi]
    0xfc, // cld
    0xf4, // hlt
];

const REP_STOSQ_FILL: &[u8] = &[
    0xfc, // cld
    0x48, 0xb8, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
    0x41, // movabs rax, 0x4141414141414141
    0xb9, 0x04, 0x00, 0x00, 0x00, // mov ecx, 0x4
    0xf3, 0x48, 0xab, // rep  stosq qword ptr es:[rdi], rax
    0xf4, // hlt
];

const STRLEN_REPNE_SCASB: &[u8] = &[
    0xfc, // cld
    0x31, 0xc0, // xor eax, eax
    0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // mov rcx, -0x1
    0xf2, 0xae, // repne  scasb al, byte ptr es:[rdi]
    0x48, 0xf7, 0xd1, // not rcx
    0x48, 0xff, 0xc9, // dec rcx
    0xf4, // hlt
];

const MEMCMP_REPE_CMPSB: &[u8] = &[
    0xfc, // cld
    0xb9, 0x08, 0x00, 0x00, 0x00, // mov ecx, 0x8
    0xf3, 0xa6, // rep  cmpsb byte ptr [rsi], byte ptr es:[rdi]
    0x0f, 0x94, 0xc0, // sete al
    0x48, 0x89, 0xcb, // mov rbx, rcx
    0xf4, // hlt
];

const LODS_STOS_UPPERCASE: &[u8] = &[
    0xfc, // cld
    0xb9, 0x05, 0x00, 0x00, 0x00, // mov ecx, 0x5
    0xac, // lodsb al, byte ptr [rsi]
    0x2c, 0x20, // sub al, 0x20
    0xaa, // stosb byte ptr es:[rdi], al
    0xe2, 0xfa, // loop 0x6
    0xf4, // hlt
];

const XLAT_TABLE: &[u8] = &[
    0xb0, 0x05, // mov al, 0x5
    0xd7, // xlatb
    0xf4, // hlt
];

const CMPXCHG8B_16B: &[u8] = &[
    0xb8, 0x11, 0x11, 0x11, 0x11, // mov eax, 0x11111111
    0xba, 0x22, 0x22, 0x22, 0x22, // mov edx, 0x22222222
    0xbb, 0x33, 0x33, 0x33, 0x33, // mov ebx, 0x33333333
    0xb9, 0x44, 0x44, 0x44, 0x44, // mov ecx, 0x44444444
    0x0f, 0xc7, 0x0e, // cmpxchg8b qword ptr [rsi]
    0x41, 0x0f, 0x94, 0xc0, // sete r8b
    0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 0x1
    0x48, 0xc7, 0xc2, 0x02, 0x00, 0x00, 0x00, // mov rdx, 0x2
    0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // mov rbx, 0x3
    0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // mov rcx, 0x4
    0x48, 0x0f, 0xc7, 0x0f, // cmpxchg16b xmmword ptr [rdi]
    0x41, 0x0f, 0x94, 0xc1, // sete r9b
    0xf4, // hlt
];

const BT_BIT_ARRAY: &[u8] = &[
    0xb8, 0x64, 0x00, 0x00, 0x00, // mov eax, 0x64
    0x0f, 0xab, 0x06, // bts dword ptr [rsi], eax
    0xb8, 0x64, 0x00, 0x00, 0x00, // mov eax, 0x64
    0x0f, 0xa3, 0x06, // bt dword ptr [rsi], eax
    0x0f, 0x92, 0xc3, // setb bl
    0xb8, 0x65, 0x00, 0x00, 0x00, // mov eax, 0x65
    0x0f, 0xa3, 0x06, // bt dword ptr [rsi], eax
    0x0f, 0x92, 0xc7, // setb bh
    0xf4, // hlt
];

const MOVBE_AND_CRC32: &[u8] = &[
    0x0f, 0x38, 0xf0, 0x06, // movbe eax, dword ptr [rsi]
    0x48, 0x0f, 0x38, 0xf1, 0x46, 0x08, // movbe qword ptr [rsi + 0x8], rax
    0x31, 0xd2, // xor edx, edx
    0xf2, 0x0f, 0x38, 0xf0, 0x16, // crc32 edx, byte ptr [rsi]
    0xb9, 0x00, 0x00, 0x00, 0x00, // mov ecx, 0x0
    0xf2, 0x0f, 0x38, 0xf1, 0xc8, // crc32 ecx, eax
    0xf4, // hlt
];

const SELF_CHECKSUM_LOOP: &[u8] = &[
    0x31, 0xc0, // xor eax, eax
    0xb9, 0x08, 0x00, 0x00, 0x00, // mov ecx, 0x8
    0x0f, 0xb6, 0x16, // movzx edx, byte ptr [rsi]
    0x01, 0xd0, // add eax, edx
    0xc1, 0xc0, 0x03, // rol eax, 0x3
    0x48, 0xff, 0xc6, // inc rsi
    0xff, 0xc9, // dec ecx
    0x75, 0xf1, // jne 0x7
    0xf4, // hlt
];

const BUBBLE_SORT: &[u8] = &[
    0xb9, 0x07, 0x00, 0x00, 0x00, // mov ecx, 0x7
    0x31, 0xd2, // xor edx, edx
    0x48, 0x89, 0xf7, // mov rdi, rsi
    0x8b, 0x07, // mov eax, dword ptr [rdi]
    0x3b, 0x47, 0x04, // cmp eax, dword ptr [rdi + 0x4]
    0x76, 0x0a, // jbe 0x1b
    0x87, 0x47, 0x04, // xchg dword ptr [rdi + 0x4], eax
    0x89, 0x07, // mov dword ptr [rdi], eax
    0xba, 0x01, 0x00, 0x00, 0x00, // mov edx, 0x1
    0x48, 0x83, 0xc7, 0x04, // add rdi, 0x4
    0x48, 0x8d, 0x04, 0x8e, // lea rax, [rsi + 4*rcx]
    0x48, 0x39, 0xc7, // cmp rdi, rax
    0x72, 0xe2, // jb 0xa
    0x85, 0xd2, // test edx, edx
    0x75, 0xd9, // jne 0x5
    0xf4, // hlt
];

#[test]
fn addressing_modes_read_and_write() {
    let e = run_with_data(ADDRESSING_MODES, &[0; 16], |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rcx = 2;
    });
    assert_eq!(e.regs().rax, 0x4400_0011);
    assert_eq!(e.regs().rdx, 0x0000_0022_0000_3333, "unaligned qword load");
    assert_eq!(e.regs().rdi, 0x22);
    assert_eq!(
        data(&e, 0, 12),
        [
            0x11, 0x01, 0x00, 0x44, 0x33, 0x33, 0x00, 0x00, 0x22, 0x01, 0x00, 0x00
        ]
    );
}

#[test]
fn push_and_pop_memory_operands() {
    let qword = 0xaaaa_aaaa_bbbb_bbbbu64.to_le_bytes();
    let e = run_with_data(PUSH_POP_MEMORY, &qword, |emu| emu.regs_mut().rsi = BUF);
    assert_eq!(data(&e, 8, 8), qword, "pop qword ptr [rsi + 8]");
    assert_eq!(e.regs().rax, BUF);
}

#[test]
fn compare_and_swap_loop() {
    let e = run_with_data(CAS_LOOP, &100u64.to_le_bytes(), |emu| {
        emu.regs_mut().rsi = BUF
    });
    assert_eq!(
        data(&e, 0, 8),
        135u64.to_le_bytes(),
        "five successful +7 updates"
    );
    assert_eq!(e.regs().rcx, 0);
    assert_eq!(e.regs().rax, 128, "rax keeps the last compared value");
}

#[test]
fn rep_movsb_forward_copy() {
    let e = run_with_data(REP_MOVSB_FORWARD, b"0123456789abcdef", |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 32;
    });
    assert_eq!(data(&e, 32, 16), b"0123456789abcdef");
    assert_eq!(
        (e.regs().rsi, e.regs().rdi, e.regs().rcx),
        (BUF + 16, BUF + 48, 0)
    );
}

#[test]
fn rep_movsb_backward_overlapping_copy_is_a_memmove() {
    let e = run_with_data(REP_MOVSB_OVERLAP_BACKWARD, b"0123456789abcdef", |emu| {
        emu.regs_mut().rsi = BUF + 15;
        emu.regs_mut().rdi = BUF + 23;
    });
    assert_eq!(
        data(&e, 8, 16),
        b"0123456789abcdef",
        "shifted by 8 without corruption"
    );
    assert_eq!(
        data(&e, 0, 8),
        b"01234567",
        "bytes before the destination untouched"
    );
    assert_eq!((e.regs().rsi, e.regs().rdi), (BUF - 1, BUF + 7));
    assert!(!e.flags_snapshot().f_df, "cld at the end");
}

#[test]
fn rep_stosq_fills_qwords() {
    let e = run_with_data(REP_STOSQ_FILL, &[0; 64], |emu| {
        emu.regs_mut().rdi = BUF + 16
    });
    assert_eq!(data(&e, 0, 16), [0; 16]);
    assert_eq!(data(&e, 16, 32), [0x41; 32]);
    assert_eq!(data(&e, 48, 8), [0; 8]);
    assert_eq!(e.regs().rdi, BUF + 48);
}

#[test]
fn strlen_with_repne_scasb() {
    let e = run_with_data(STRLEN_REPNE_SCASB, b"hello world\0", |emu| {
        emu.regs_mut().rdi = BUF
    });
    assert_eq!(e.regs().rcx, 11);
    assert_eq!(e.regs().rdi, BUF + 12, "rdi stops after the terminator");
}

#[test]
fn memcmp_with_repe_cmpsb() {
    let mut equal = [0u8; 24];
    equal[..8].copy_from_slice(b"abcdefgh");
    equal[16..].copy_from_slice(b"abcdefgh");
    let e = run_with_data(MEMCMP_REPE_CMPSB, &equal, |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 16;
    });
    assert_eq!(e.regs().rax & 0xff, 1, "equal: ZF set at the end");
    assert_eq!(e.regs().rbx, 0, "all eight compared");

    let mut differ = equal;
    differ[20] = b'X';
    let e = run_with_data(MEMCMP_REPE_CMPSB, &differ, |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 16;
    });
    assert_eq!(e.regs().rax & 0xff, 0, "mismatch at the fifth byte");
    assert_eq!(e.regs().rbx, 3, "three elements left");
    assert_eq!((e.regs().rsi, e.regs().rdi), (BUF + 5, BUF + 21));
}

#[test]
fn lodsb_stosb_loop_uppercases_a_string() {
    let e = run_with_data(LODS_STOS_UPPERCASE, b"hello", |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 16;
    });
    assert_eq!(data(&e, 16, 5), b"HELLO");
    assert_eq!(e.regs().rax & 0xff, b'O' as u64);
}

#[test]
fn xlatb_table_lookup() {
    let table: Vec<u8> = (0u8..16).map(|i| i * 2).collect();
    let e = run_with_data(XLAT_TABLE, &table, |emu| emu.regs_mut().rbx = BUF);
    assert_eq!(e.regs().rax, 10);
}

#[test]
fn cmpxchg8b_and_cmpxchg16b() {
    let mut mem = [0u8; 32];
    mem[..8].copy_from_slice(&0x2222_2222_1111_1111u64.to_le_bytes());
    mem[16..24].copy_from_slice(&1u64.to_le_bytes());
    mem[24..].copy_from_slice(&2u64.to_le_bytes());
    let e = run_with_data(CMPXCHG8B_16B, &mem, |emu| {
        emu.regs_mut().rsi = BUF;
        emu.regs_mut().rdi = BUF + 16;
    });
    assert_eq!(
        data(&e, 0, 8),
        0x4444_4444_3333_3333u64.to_le_bytes(),
        "ecx:ebx stored"
    );
    assert_eq!(data(&e, 16, 8), 3u64.to_le_bytes(), "rbx stored low");
    assert_eq!(data(&e, 24, 8), 4u64.to_le_bytes(), "rcx stored high");
    assert_eq!((e.regs().r8, e.regs().r9), (1, 1), "both compares matched");
}

#[test]
fn bit_test_on_a_memory_bit_array() {
    let e = run_with_data(BT_BIT_ARRAY, &[0; 16], |emu| emu.regs_mut().rsi = BUF);
    assert_eq!(data(&e, 12, 1), [0x10], "bit 100 lives in byte 12");
    assert_eq!(e.regs().rbx, 0x0001, "bit 100 set, bit 101 clear");
}

#[test]
fn movbe_and_crc32() {
    let e = run_with_data(MOVBE_AND_CRC32, &[0x78, 0x56, 0x34, 0x12], |emu| {
        emu.regs_mut().rsi = BUF
    });
    assert_eq!(e.regs().rax, 0x7856_3412, "big-endian load");
    assert_eq!(
        data(&e, 8, 8),
        [0, 0, 0, 0, 0x78, 0x56, 0x34, 0x12],
        "big-endian store"
    );
    assert_eq!(e.regs().rdx, 0xfb41_0cc2, "crc32 of one byte");
    assert_eq!(e.regs().rcx, 0x0b67_da4d, "crc32 of a dword");
}

#[test]
fn checksum_loop_over_bytes() {
    let e = run_with_data(SELF_CHECKSUM_LOOP, &[1, 2, 3, 4, 5, 6, 7, 8], |emu| {
        emu.regs_mut().rsi = BUF
    });
    assert_eq!(e.regs().rax, 0x014e_5e00);
    assert_eq!(e.regs().rsi, BUF + 8);
}

#[test]
fn bubble_sort_in_memory() {
    let input: Vec<u8> = [5u32, 3, 8, 1, 9, 2, 7, 4]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let e = run_with_data(BUBBLE_SORT, &input, |emu| emu.regs_mut().rsi = BUF);
    let sorted: Vec<u8> = [1u32, 2, 3, 4, 5, 7, 8, 9]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    assert_eq!(data(&e, 0, 32), sorted);
}
