//! Helpers for the hand-crafted shellcode tests (`shellcode_*.rs`): every test
//! assembles a small x86-64 program, runs it to `hlt` and checks registers,
//! flags and memory against what the CPU produces. No sample binaries needed.

use crate::maps::mem64::Permission;
use crate::*;
/// Data buffer the memory tests map read-write.
pub const BUF: u64 = 0x3000_0000;

/// Load `code` and let `setup` prepare registers and memory, without running.
pub fn prepare(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    setup(&mut emu);
    emu
}

/// Load `code`, prepare it with `setup` and run it to `hlt`.
pub fn run(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = prepare(code, setup);
    emu.run(None).unwrap();
    emu
}

/// Like `run`, with `data` mapped at `BUF` first.
pub fn run_with_data(code: &[u8], data: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    run(code, |emu| {
        emu.maps
            .create_map("buf", BUF, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_bytes(BUF, data);
        setup(emu);
    })
}

/// The bytes at `BUF + off`.
pub fn data(emu: &Emu, off: u64, len: usize) -> Vec<u8> {
    emu.maps.read_bytes(BUF + off, len).to_vec()
}
