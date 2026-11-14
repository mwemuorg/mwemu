use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsPrefixA(emu: &mut emu::Emu) {
    path_is_prefix(emu, "PathIsPrefixA", false);
}

fn path_is_prefix(emu: &mut emu::Emu, api: &str, wide: bool) {
    let prefix_ptr = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let prefix = if prefix_ptr != 0 {
        strings::read_text(emu, prefix_ptr, wide)
    } else {
        String::new()
    };
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_prefix = path.to_lowercase().starts_with(&prefix.to_lowercase());
    log_red!(
        emu,
        "kernelbase!{api} '{}' of '{}' => {}",
        prefix,
        path,
        is_prefix
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_prefix { 1 } else { 0 };
}

pub fn PathIsPrefixW(emu: &mut emu::Emu) {
    path_is_prefix(emu, "PathIsPrefixW", true);
}
