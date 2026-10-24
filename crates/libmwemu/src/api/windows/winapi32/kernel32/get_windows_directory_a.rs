use crate::emu;

pub fn GetWindowsDirectoryA(emu: &mut emu::Emu) {
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!GetWindowsDirectoryA");

    emu.maps.write_string(ptr, "C:\\Windows");
    emu.regs_mut().rax = size;

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
