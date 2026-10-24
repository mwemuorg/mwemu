use crate::emu;

pub fn GetFileSize(emu: &mut emu::Emu) {
    let file_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpFileSizeHigh = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "** {} kernel32!GetFileSize file_handle: 0x{:x}",
        emu.pos,
        file_handle
    );

    let Some(file_handle) = emu.handle_management.get_mut_file_handle(file_handle) else {
        log::warn!(
            "kernel32!GetFileSize: invalid file handle 0x{:x}",
            file_handle
        );
        emu.regs_mut().set_eax(0xFFFFFFFF); // INVALID_FILE_SIZE
        return;
    };
    let file_size = file_handle.file_size;
    emu.regs_mut().set_eax(file_size);
    if lpFileSizeHigh != 0x0 {
        emu.maps
            .write_dword(lpFileSizeHigh as u64, (file_size >> 32) as u32);
    }
}
