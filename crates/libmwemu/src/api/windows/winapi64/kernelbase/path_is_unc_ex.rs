use crate::emu;

pub fn PathIsUNCEx(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let server_ptr_ptr = emu.regs().rdx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_unc = path.starts_with("\\\\");
    if is_unc && server_ptr_ptr != 0 {
        emu.maps.write_qword(server_ptr_ptr, path_ptr + 4);
    }
    log_red!(emu, "kernelbase!PathIsUNCEx '{}' => {}", path, is_unc);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_unc { 1 } else { 0 };
}
