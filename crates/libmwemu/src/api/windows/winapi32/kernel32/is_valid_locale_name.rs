use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsValidLocaleName(emu: &mut emu::Emu) {
    let locale_name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_wide_string(locale_name_ptr);
    let valid = !name.is_empty();
    log_red!(emu, "kernel32!IsValidLocaleName '{}' => {}", name, valid);
    emu.set_last_error(0);
    emu.regs_mut().rax = if valid { 1 } else { 0 };
    stdcall_ret(emu, 1);
}
