use crate::emu;

pub fn _wrename(emu: &mut emu::Emu) {
    let old_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let new_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let old_name = emu.maps.read_wide_string(old_ptr);
    let new_name = emu.maps.read_wide_string(new_ptr);
    log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
    emu.regs_mut().rax = 0;
}
