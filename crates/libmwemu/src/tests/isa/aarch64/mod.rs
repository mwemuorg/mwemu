mod aarch64_basic;
mod aarch64_compiler_ops;
mod aarch64_fp_simd;

use crate::emu::Emu;
use crate::maps::mem64::Permission;

/// Build an AArch64 emulator with `words` (little-endian instruction
/// encodings, as printed by `objdump -d`) loaded at the code base.
pub fn emu_with(words: &[u32]) -> Emu {
    crate::tests::helpers::setup();
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let mut emu = crate::emu_aarch64();
    emu.load_code_bytes(&bytes);
    emu
}

/// Map a zeroed, writable 4 KiB scratch region and return its base.
pub fn data_map(emu: &mut Emu) -> u64 {
    let base = emu.maps.alloc(0x1000).expect("cannot reserve test data");
    emu.maps
        .create_map("testdata", base, 0x1000, Permission::READ_WRITE)
        .expect("cannot create test data map");
    base
}

/// Execute one instruction, failing the test if it is not emulated.
pub fn step_ok(emu: &mut Emu) {
    let pc = emu.regs_aarch64().pc;
    assert!(emu.step(), "instruction at 0x{:x} was not emulated", pc);
}
