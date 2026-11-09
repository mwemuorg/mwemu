use crate::emu;

pub fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.regs().rcx as u16;

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.regs_mut().rax = port.to_be() as u64;
}
