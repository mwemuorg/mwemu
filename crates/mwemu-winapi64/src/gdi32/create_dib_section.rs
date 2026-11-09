use crate::emu;
use crate::winapi::helper;

pub fn CreateDIBSection(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let pbmi = emu.regs().rdx;
    let usage = emu.regs().r8;
    let ppv_bits = emu.regs().r9;
    log_red!(
        emu,
        "gdi32!CreateDIBSection hdc:0x{:x} bmi:0x{:x} usage:{} bits_out:0x{:x}",
        hdc,
        pbmi,
        usage,
        ppv_bits
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://bitmap");
}
