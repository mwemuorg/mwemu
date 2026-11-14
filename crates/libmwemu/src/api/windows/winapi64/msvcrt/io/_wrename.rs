use crate::emu;

pub fn _wrename(emu: &mut emu::Emu) {
    let old_ptr = emu.regs().rcx;
    let new_ptr = emu.regs().rdx;
    let old_name = emu.maps.read_wide_string(old_ptr);
    let new_name = emu.maps.read_wide_string(new_ptr);
    log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
