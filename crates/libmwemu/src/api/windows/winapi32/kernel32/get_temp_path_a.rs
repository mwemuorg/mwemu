use crate::emu;

pub fn GetTempPathA(emu: &mut emu::Emu) {
    let bufflen = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let buff_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if bufflen >= 7 {
        emu.maps.write_string(buff_ptr, "c:\\tmp\\");
        emu.regs_mut().rax = 6;
    } else {
        emu.regs_mut().rax = 0;
    }

    log_red!(emu, "kernel32!GetTempPathA");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
