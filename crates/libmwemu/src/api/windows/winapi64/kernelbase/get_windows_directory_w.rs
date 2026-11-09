use crate::emu;

pub fn GetWindowsDirectoryW(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let dir = "C:\\Windows";
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, dir);
    }
    log_red!(emu, "kernelbase!GetWindowsDirectoryW => '{}'", dir);
    emu.set_last_error(0);
    emu.regs_mut().rax = dir.len() as u64;
}
