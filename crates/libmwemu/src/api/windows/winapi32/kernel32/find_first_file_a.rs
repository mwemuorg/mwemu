use crate::emu;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let find_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let file = emu.maps.read_string(file_ptr);

    log_red!(emu, "kernel32!FindFirstFileA file: {}", file);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
