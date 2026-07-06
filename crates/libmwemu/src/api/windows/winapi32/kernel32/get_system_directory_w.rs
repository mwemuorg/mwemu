use crate::emu;

pub fn GetSystemDirectoryW(emu: &mut emu::Emu) {
    let out_buff_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetSystemDirectoryW cannot read out_buff_ptr param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetSystemDirectoryW cannot read size param");

    emu.maps
        .write_wide_string(out_buff_ptr, "C:\\Windows\\\x00\x00");

    log_red!(emu, "kernel32!GetSystemDirectoryW");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 11; // * 2;
}
