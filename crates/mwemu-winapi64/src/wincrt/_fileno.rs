use crate::emu;

pub fn _fileno(emu: &mut emu::Emu) {
    let stream = emu.regs().rcx;
    log_red!(emu, "wincrt!_fileno stream:0x{:x}", stream);
    let _ = emu.maps.is_mapped(stream);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
