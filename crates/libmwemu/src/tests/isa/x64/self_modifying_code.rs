//! Self-modifying code must execute the bytes that are in memory, not a stale
//! decoded copy. Real x86 keeps the instruction stream coherent with stores
//! (the SDM calls it "self-modifying code" handling), and packers rely on it:
//! they decrypt a buffer with `rep movs`/`movdqu`/an API call and jump into it.
//! Every store path that can land in executable memory is covered here.

use crate::*;

fn run_code(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    emu.run(None).unwrap();
    emu
}

/// 16-byte stub: `mov eax, imm32 ; nop x10 ; ret`.
fn stub(imm: u8) -> [u8; 16] {
    let mut s = [0x90u8; 16];
    s[0] = 0xb8;
    s[1] = imm;
    s[2] = 0;
    s[3] = 0;
    s[4] = 0;
    s[15] = 0xc3;
    s
}

/// A store to the instruction right after the current one must be observed:
/// `mov byte [rip+1], 2` patches the immediate of the following `mov eax, 1`.
#[test]
fn store_to_next_instruction_is_executed() {
    let mut code = vec![
        0xc6, 0x05, 0x01, 0x00, 0x00, 0x00, 0x02, // mov byte ptr [rip+1], 2
        0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 1   (imm byte at offset 8)
        0xf4, // hlt
    ];
    // The block decoder stops 16 bytes before the end of the map; pad so the
    // three instructions above land in one cached block.
    code.extend(std::iter::repeat_n(0x90u8, 0x400));
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().get_eax(),
        2,
        "the patched immediate must be executed, not the cached decode"
    );
}

/// Layout shared by the `rep movsb` tests: call stub, copy `count` bytes of a
/// new stub over it, call it again.
fn rep_movsb_program(count: u8) -> Vec<u8> {
    let mut code = vec![
        0xe8, 0x1b, 0x00, 0x00, 0x00, // call target (0x20)
        0x48, 0x8d, 0x35, 0x24, 0x00, 0x00, 0x00, // lea rsi, [rip+0x24] -> newbytes (0x30)
        0x48, 0x8d, 0x3d, 0x0d, 0x00, 0x00, 0x00, // lea rdi, [rip+0x0d] -> target (0x20)
        0xb9, count, 0x00, 0x00, 0x00, // mov ecx, count
        0xf3, 0xa4, // rep movsb
        0xe8, 0x01, 0x00, 0x00, 0x00, // call target
        0xf4, // hlt
    ];
    assert_eq!(code.len(), 0x20);
    code.extend_from_slice(&stub(1)); // 0x20: target
    code.extend_from_slice(&stub(2)); // 0x30: newbytes
    code
}

/// `rep movsb` with >= 16 elements takes the bulk fast path, which writes the
/// destination map directly.
#[test]
fn rep_movsb_bulk_overwrite_is_executed() {
    let emu = run_code(&rep_movsb_program(16));
    assert_eq!(emu.regs().get_eax(), 2, "bulk rep movsb rewrote the stub");
}

/// Short `rep movsb` goes through the per-element `movsb` handler.
#[test]
fn rep_movsb_per_element_overwrite_is_executed() {
    let emu = run_code(&rep_movsb_program(6));
    assert_eq!(
        emu.regs().get_eax(),
        2,
        "per-element rep movsb rewrote the stub"
    );
}

/// A 128-bit vector store (`movdqu [mem], xmm`) over already-executed code.
#[test]
fn movdqu_overwrite_is_executed() {
    let mut code = vec![
        0xe8, 0x1b, 0x00, 0x00, 0x00, // call target (0x20)
        0xf3, 0x0f, 0x6f, 0x05, 0x23, 0x00, 0x00, 0x00, // movdqu xmm0, [rip+0x23] (newbytes)
        0xf3, 0x0f, 0x7f, 0x05, 0x0b, 0x00, 0x00, 0x00, // movdqu [rip+0x0b], xmm0 (target)
        0xe8, 0x06, 0x00, 0x00, 0x00, // call target
        0xf4, // hlt
        0x90, 0x90, 0x90, 0x90, 0x90, // pad to 0x20
    ];
    assert_eq!(code.len(), 0x20);
    code.extend_from_slice(&stub(1));
    code.extend_from_slice(&stub(2));
    let emu = run_code(&code);
    assert_eq!(emu.regs().get_eax(), 2, "movdqu store rewrote the stub");
}

/// Host-side writes (what an API stub such as ReadFile/WriteProcessMemory does)
/// land through `Maps::write_bytes` while the run loop is active. Simulate one
/// from a pre-instruction hook right before the second call.
#[test]
fn host_write_from_hook_is_executed() {
    let mut code = vec![
        0xe8, 0x0b, 0x00, 0x00, 0x00, // call target (0x10)
        0xe8, 0x06, 0x00, 0x00, 0x00, // call target
        0xf4, // hlt
        0x90, 0x90, 0x90, 0x90, 0x90, // pad to 0x10
    ];
    assert_eq!(code.len(), 0x10);
    code.extend_from_slice(&stub(1));

    let mut emu = emu64();
    emu.load_code_bytes(&code);
    let base = emu.pc();
    emu.set_hook_on_pre_instruction(move |emu, pc, _ins, _sz| {
        if pc == base + 5 {
            emu.maps.write_bytes(base + 0x10, &stub(2));
        }
        true
    });
    emu.run(None).unwrap();
    assert_eq!(emu.regs().get_eax(), 2, "host write rewrote the stub");
}

/// Two blocks in a page whose index ends in 0xffff used to probe past the end
/// of the cache-entry table (index out of bounds panic).
#[test]
fn instruction_cache_high_page_index_does_not_panic() {
    let code = [
        0xeb, 0x02, // jmp +2   (ends block 1)
        0x90, 0x90, // nop nop
        0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 1  (block 2, same page)
        0xf4, // hlt
    ];
    let mut emu = emu64();
    emu.cfg.code_base_addr = 0x0fff_f000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().get_eax(), 1);
}
