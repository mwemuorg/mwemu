//! Helpers to call Linux/macOS API stubs directly, the way the engine does
//! after intercepting a call into a stub library.
use crate::emu::Emu;
use crate::maps::mem64::Permission;

/// Map a zeroed RW scratch region of `size` bytes and return its base.
pub fn scratch(emu: &mut Emu, name: &str, size: u64) -> u64 {
    let base = emu.maps.alloc(size).expect("cannot reserve scratch");
    emu.maps
        .create_map(name, base, size, Permission::READ_WRITE)
        .expect("cannot create scratch map");
    base
}

/// AArch64 emulator with a mapped stack (sp in the middle) and data area.
pub fn aarch64_env() -> (Emu, u64) {
    crate::tests::helpers::setup();
    let mut emu = crate::emu_aarch64();
    let stack = scratch(&mut emu, "api_stack", 0x10000);
    emu.regs_aarch64_mut().sp = stack + 0x8000;
    let data = scratch(&mut emu, "api_data", 0x10000);
    (emu, data)
}

/// Write a NUL-terminated string and return its address.
pub fn put_str(emu: &mut Emu, addr: u64, s: &str) -> u64 {
    emu.maps.write_bytes(addr, s.as_bytes());
    emu.maps.write_byte(addr + s.len() as u64, 0);
    addr
}

/// Place integer arguments in x0..x7 and return nothing.
pub fn set_args(emu: &mut Emu, args: &[u64]) {
    for (i, a) in args.iter().enumerate() {
        emu.regs_aarch64_mut().x[i] = *a;
    }
}

pub fn ret(emu: &Emu) -> u64 {
    emu.regs_aarch64().x[0]
}

pub fn stdout_text(emu: &Emu) -> String {
    String::from_utf8_lossy(&emu.emulated_stdout).into_owned()
}
