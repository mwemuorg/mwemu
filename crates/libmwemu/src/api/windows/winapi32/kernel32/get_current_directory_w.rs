use crate::emu;

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    let buff_len = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetCurrentDirectoryW cannot read buff_len");
    let buff_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetCurrentDirectoryW cannot read buff_ptr") as u64;

    emu.maps
        .write_string(buff_ptr, "c\x00:\x00\\\x00\x00\x00\x00\x00");

    log_red!(emu, "kernel32!GetCurrentDirectoryW");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 6;
}
