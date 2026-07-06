use crate::emu;

pub fn GetTempPathW(emu: &mut emu::Emu) {
    let bufflen = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetTempPathW cannot read bufflen");
    let buff_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetTempPathW cannot read buff_ptr") as u64;

    if bufflen >= 14 {
        emu.maps.write_wide_string(buff_ptr, "c:\\tmp\\");
        emu.regs_mut().rax = 14;
    } else {
        emu.regs_mut().rax = 0;
    }

    log_red!(emu, "kernel32!GetTempPathW");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
