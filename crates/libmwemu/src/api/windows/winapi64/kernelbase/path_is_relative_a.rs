use crate::emu;

pub fn PathIsRelativeA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    let is_relative = !path.starts_with("\\") && !path.contains(":\\") && !path.starts_with("/");
    log_red!(
        emu,
        "kernelbase!PathIsRelativeA '{}' => {}",
        path,
        is_relative
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_relative { 1 } else { 0 };
}
