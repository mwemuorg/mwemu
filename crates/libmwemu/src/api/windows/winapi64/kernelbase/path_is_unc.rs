use crate::emu;

pub fn PathIsUNCA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let is_unc = path.starts_with("\\\\");
    log_red!(emu, "kernelbase!PathIsUNCA '{}' => {}", path, is_unc);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_unc { 1 } else { 0 };
}

pub fn PathIsUNCW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_unc = path.starts_with("\\\\");
    log_red!(emu, "kernelbase!PathIsUNCW '{}' => {}", path, is_unc);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_unc { 1 } else { 0 };
}
