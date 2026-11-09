use crate::emu;

pub fn GetFileInformationByName(emu: &mut emu::Emu) {
    let file_name_ptr = emu.regs().rcx;
    let info_class = emu.regs().rdx as u32;
    let info_ptr = emu.regs().r8;
    let info_size = emu.regs().r9 as u32;
    let file_name = emu.maps.read_wide_string(file_name_ptr);
    log_red!(
        emu,
        "kernel32!GetFileInformationByName '{}' class:{}",
        file_name,
        info_class
    );
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
}
