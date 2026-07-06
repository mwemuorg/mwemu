use crate::emu;

pub fn GetFileSize(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx as u32;
    let lpFileSizeHigh = emu.regs().rdx;

    log_red!(
        emu,
        "** {} kernel32!GetFileSize file_handle: 0x{:x}",
        emu.pos,
        file_handle
    );

    let file_handle = emu
        .handle_management
        .get_mut_file_handle(file_handle)
        .expect("Failed to get file handle");
    let file_size = file_handle.file_size;
    emu.regs_mut().set_eax(file_size);
    if lpFileSizeHigh != 0x0 {
        emu.maps
            .write_dword(lpFileSizeHigh, (file_size >> 32) as u32);
    }
}
