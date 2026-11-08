use crate::emu;

pub fn PathIsURLW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_url = path.contains("://");
    log_red!(emu, "kernelbase!PathIsURLW '{}' => {}", path, is_url);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_url { 1 } else { 0 };
}
