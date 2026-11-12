use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _wrename(emu: &mut emu::Emu) {
    let old_ptr = WinApiAbi::arg(emu, 0);
    let new_ptr = WinApiAbi::arg(emu, 1);
    let old_name = emu.maps.read_wide_string(old_ptr);
    let new_name = emu.maps.read_wide_string(new_ptr);
    log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
    emu.regs_mut().rax = 0;
}
