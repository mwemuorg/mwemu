use crate::emu;

pub fn _putenv_s(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let name = emu.maps.read_string(name_ptr);
    let value = emu.maps.read_string(value_ptr);
    log_red!(emu, "wincrt!_putenv_s '{}={}'", name, value);
    emu.regs_mut().rax = 0;
}
