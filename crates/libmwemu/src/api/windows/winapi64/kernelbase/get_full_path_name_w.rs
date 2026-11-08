use crate::emu;

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let filepart_ptr = emu.regs().r9;
    let path = emu.maps.read_wide_string(path_ptr);
    let full = format!("C:\\mwemu\\{}", path);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, &full);
    }
    log_red!(emu, "kernelbase!GetFullPathNameW '{}' => '{}'", path, full);
    emu.set_last_error(0);
    emu.regs_mut().rax = full.len() as u64;
}
