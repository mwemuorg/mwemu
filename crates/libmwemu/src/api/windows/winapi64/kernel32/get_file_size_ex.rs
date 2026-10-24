use crate::emu;

pub fn GetFileSizeEx(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_file_size_high = emu.regs().rdx;
    log_red!(
        emu,
        "** {} kernel32!GetFileSize {:x} {:x}",
        emu.pos,
        h_file,
        lp_file_size_high
    );

    let Some(file_handle) = emu.handle_management.get_mut_file_handle(h_file as u32) else {
        log::warn!("kernel32!GetFileSizeEx: invalid file handle 0x{:x}", h_file);
        emu.regs_mut().rax = 0; // FALSE
        return;
    };
    let file_size = file_handle.file_size;
    emu.regs_mut().set_eax(file_size);
    if lp_file_size_high != 0x0 {
        emu.maps.write_qword(lp_file_size_high, file_size);
    }
}
