use crate::emu;

pub fn RemoveDirectoryA(emu: &mut emu::Emu) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!RemoveDirectoryA '{}'", path);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
