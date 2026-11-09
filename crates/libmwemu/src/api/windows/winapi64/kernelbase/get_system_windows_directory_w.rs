use crate::emu;

pub fn GetSystemWindowsDirectoryW(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let buf_size = emu.regs().rdx as usize;
    let windir = "C:\\Windows";
    log_red!(emu, "kernelbase!GetSystemWindowsDirectoryW => '{}'", windir);
    if buf != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf, windir);
    }
    emu.regs_mut().rax = windir.len() as u64;
}
