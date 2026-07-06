use crate::emu;

pub fn GetSystemWindowsDirectoryW(emu: &mut emu::Emu) {
    let ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemWindowsDirectoryW: error reading param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetSystemWindowsDirectoryW: error reading param") as u64;

    log_red!(emu, "kernel32!GetSystemWindowsDirectoryW");

    emu.maps.write_wide_string(ptr, "C:\\Windows\\system32\\");
    emu.regs_mut().rax = size;

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
