use crate::emu;

pub fn IsApiSetImplemented(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "kernelbase!IsApiSetImplemented '{}' => TRUE", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
