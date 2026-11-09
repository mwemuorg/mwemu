use crate::emu;

pub fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.maps.read_word(emu.regs().get_esp()).unwrap_or_default();

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.stack_pop32(false);
    emu.regs_mut().rax = port.to_be() as u64;
}
