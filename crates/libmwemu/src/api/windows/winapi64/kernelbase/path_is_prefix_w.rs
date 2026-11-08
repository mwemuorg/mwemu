use crate::emu;

pub fn PathIsPrefixW(emu: &mut emu::Emu) {
    let prefix_ptr = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let prefix = if prefix_ptr != 0 {
        emu.maps.read_wide_string(prefix_ptr)
    } else {
        String::new()
    };
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_prefix = path.to_lowercase().starts_with(&prefix.to_lowercase());
    log_red!(
        emu,
        "kernelbase!PathIsPrefixW '{}' of '{}' => {}",
        prefix,
        path,
        is_prefix
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_prefix { 1 } else { 0 };
}
