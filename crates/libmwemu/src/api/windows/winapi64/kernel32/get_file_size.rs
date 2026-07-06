use crate::emu;

pub fn GetFileSize(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_file_size_high = emu.regs().rdx;
    log_red!(
        emu,
        "** {} kernel32!GetFileSize {:x} {:x}",
        emu.pos,
        h_file,
        lp_file_size_high
    );

    let file_handle = emu
        .handle_management
        .get_mut_file_handle(h_file as u32)
        .expect("Failed to get file handle");
    let file_size = file_handle.file_size;
    emu.regs_mut().set_eax(file_size);
    if lp_file_size_high != 0x0 {
        emu.maps
            .write_dword(lp_file_size_high, (file_size >> 32) as u32);
    }
}
