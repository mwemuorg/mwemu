use crate::emu;

pub fn GetSystemDirectoryA(emu: &mut emu::Emu) {
    let out_buff_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetSystemDirectoryA cannot read out_buff_ptr param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetSystemDirectoryA cannot read size param");

    emu.maps.write_string(out_buff_ptr, "C:\\Windows\\\x00");

    log_red!(emu, "kernel32!GetSystemDirectoryA");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 11;
}
