use crate::emu;

pub fn PathIsUNCServerShareA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let is_share = if let Some(rest) = path.strip_prefix("\\\\") {
        rest.matches('\\').count() == 1 && !rest.ends_with('\\')
    } else {
        false
    };
    log_red!(
        emu,
        "kernelbase!PathIsUNCServerShareA '{}' => {}",
        path,
        is_share
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_share { 1 } else { 0 };
}

pub fn PathIsUNCServerShareW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_share = if let Some(rest) = path.strip_prefix("\\\\") {
        rest.matches('\\').count() == 1 && !rest.ends_with('\\')
    } else {
        false
    };
    log_red!(
        emu,
        "kernelbase!PathIsUNCServerShareW '{}' => {}",
        path,
        is_share
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_share { 1 } else { 0 };
}
