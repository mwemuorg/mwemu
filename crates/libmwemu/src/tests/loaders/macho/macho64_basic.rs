use crate::tests::helpers;
use crate::*;

/// Full Mach-O binary for: int main() { int a=2; int b=2; return a+b; }
/// Compiled with: cc -arch arm64 -nostdlib -static -e _main -o test main.c
const MACHO64_ADD: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_add.bin");

#[test]
fn macho64_aarch64_load_and_execute() {
    helpers::setup();

    // Write the embedded binary to a temp file so load_code() can detect it
    let tmp = std::env::temp_dir().join("mwemu_test_macho64_add.bin");
    std::fs::write(&tmp, MACHO64_ADD).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());

    // Verify architecture was detected
    assert!(emu.cfg.arch.is_aarch64());

    // Entry point should be in the __TEXT segment (0x100000000 range)
    let pc = emu.regs_aarch64().pc;
    assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);

    // The compiled C code produces 10 instructions:
    //   sub  sp, sp, #0x10
    //   str  wzr, [sp, #0xc]
    //   mov  w8, #0x2
    //   str  w8, [sp, #0x8]
    //   str  w8, [sp, #0x4]
    //   ldr  w8, [sp, #0x8]
    //   ldr  w9, [sp, #0x4]
    //   add  w0, w8, w9
    //   add  sp, sp, #0x10
    //   ret

    let sp_before = emu.regs_aarch64().sp;

    for _ in 0..10 {
        emu.step();
    }

    // w0 = 2 + 2 = 4
    assert_eq!(emu.regs_aarch64().x[0] & 0xffffffff, 4);
    // SP restored
    assert_eq!(emu.regs_aarch64().sp, sp_before);
}
