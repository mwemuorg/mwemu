use crate::emu;

pub fn GetSystemWindowsDirectoryA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let buf_size = emu.regs().rdx as usize;
    let windir = "C:\\Windows";
    log_red!(emu, "kernelbase!GetSystemWindowsDirectoryA => '{}'", windir);
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf != 0 && buf_size > 0 {
        emu.maps.write_string(buf, windir);
    }
    emu.regs_mut().rax = windir.len() as u64;
}
