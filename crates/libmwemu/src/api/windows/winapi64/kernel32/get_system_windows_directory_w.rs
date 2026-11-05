use crate::emu;

pub fn GetSystemWindowsDirectoryW(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = emu.regs().rdx;

    log_red!(emu, "kernel32!GetSystemWindowsDirectoryW");

    emu.maps.write_wide_string(ptr, "C:\\Windows\\system32\\");
    emu.regs_mut().rax = size;
}
