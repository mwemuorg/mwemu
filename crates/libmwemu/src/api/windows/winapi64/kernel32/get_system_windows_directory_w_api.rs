use crate::emu;

pub fn GetSystemWindowsDirectoryW_api(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let path = "C:\\Windows";
    log_red!(emu, "kernel32!GetSystemWindowsDirectoryW => '{}'", path);
    if buf != 0 && size > path.len() as u64 {
        emu.maps.write_wide_string(buf, path);
    }
    emu.regs_mut().rax = path.len() as u64;
}
