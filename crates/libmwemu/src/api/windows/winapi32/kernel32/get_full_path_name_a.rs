use crate::emu;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let path = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let filename = emu.maps.read_string(file_ptr);

    log_red!(emu, "kernel32!GetFullPathNameA file: {}", filename);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 10;
}
