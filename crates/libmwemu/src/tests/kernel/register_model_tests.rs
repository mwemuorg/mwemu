//! Device register model: the coherent, injectable register file that backs
//! vendor register I/O over USB control transfers (a USB driver's registers are
//! control messages, not a mapped MMIO window). Two properties matter:
//!
//! * **coherence** — a write-then-read-back returns the written value, so a
//!   driver's register init loops make progress instead of spinning on zero;
//! * **injection** — a value preset by the harness is what the driver reads, so
//!   chip-ID / version detection can be steered without per-driver emulator
//!   code (config, not code).

use crate::emu64;
use crate::kernel::KernelOs;
use crate::maps::mem64::Permission;

const BUF: u64 = 0xffffd00000000000;

fn boot() -> crate::emu::Emu {
    let mut emu = emu64();
    emu.cfg.verbose = 0;
    emu.kernel_init(KernelOs::Linux);
    emu.maps
        .create_map("reg.buf", BUF, 0x1000, Permission::READ_WRITE)
        .expect("scratch buffer");
    emu
}

#[test]
fn preset_register_is_what_an_in_transfer_reads() {
    let mut emu = boot();
    // Harness presets REG_SYS_CFG; the driver's read must observe it.
    emu.kernel_set_register(0x00f0, 0x1234_5678);
    let n = emu.kernel_usb_register_xfer(0x00f0, BUF, 4, true);
    assert_eq!(n, 4);
    assert_eq!(emu.maps.read_dword(BUF).unwrap(), 0x1234_5678);
}

#[test]
fn write_then_read_back_is_coherent() {
    let mut emu = boot();
    // An OUT transfer stores the register; a later IN transfer returns it.
    emu.maps.write_dword(BUF, 0xdead_beef);
    emu.kernel_usb_register_xfer(0x0040, BUF, 4, false);
    assert_eq!(emu.kernel_get_register(0x0040), 0xdead_beef);

    let other = BUF + 0x40;
    let n = emu.kernel_usb_register_xfer(0x0040, other, 4, true);
    assert_eq!(n, 4);
    assert_eq!(emu.maps.read_dword(other).unwrap(), 0xdead_beef);
}

#[test]
fn width_is_honored_and_unset_registers_read_zero() {
    let mut emu = boot();
    // A 1-byte read of an unset register is 0, and does not disturb neighbours.
    emu.maps.write_dword(BUF, 0xffff_ffff);
    let n = emu.kernel_usb_register_xfer(0x0002, BUF, 1, true);
    assert_eq!(n, 1);
    assert_eq!(emu.maps.read_byte(BUF).unwrap(), 0x00);
    assert_eq!(emu.maps.read_byte(BUF + 1).unwrap(), 0xff);

    // A 2-byte write stores exactly two bytes.
    emu.maps.write_dword(BUF, 0x0000_abcd);
    emu.kernel_usb_register_xfer(0xfe66, BUF, 2, false);
    assert_eq!(emu.kernel_get_register(0xfe66), 0xabcd);
}
