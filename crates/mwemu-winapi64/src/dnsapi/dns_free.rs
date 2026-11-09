use crate::emu;

pub fn DnsFree(emu: &mut emu::Emu) {
    let data = emu.regs().rcx;
    log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
}
