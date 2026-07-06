use crate::emu;

pub fn GetWindowsDirectoryW(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetWindowsDirectoryW: error reading param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetWindowsDirectoryW: error reading param") as u64;

    log_red!(emu, "kernel32!GetWindowsDirectoryW");

    emu.maps.write_wide_string(ptr, "C:\\Windows");
    emu.regs_mut().rax = size;

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
