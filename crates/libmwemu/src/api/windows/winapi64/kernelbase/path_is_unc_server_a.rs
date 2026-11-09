use crate::emu;

pub fn PathIsUNCServerA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let is_server = path.starts_with("\\\\") && !path[2..].contains('\\');
    log_red!(
        emu,
        "kernelbase!PathIsUNCServerA '{}' => {}",
        path,
        is_server
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_server { 1 } else { 0 };
}
