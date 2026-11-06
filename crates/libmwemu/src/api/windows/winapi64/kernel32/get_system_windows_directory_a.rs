use crate::emu;

pub fn GetSystemWindowsDirectoryA(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = emu.regs().rdx;

    log_red!(emu, "kernel32!GetSystemWindowsDirectoryA");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.maps.write_string(ptr, "C:\\Windows\\system32\\");
    emu.regs_mut().rax = size;
}
