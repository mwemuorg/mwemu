use crate::emu;

pub fn GetFileSize(emu: &mut emu::Emu) {
    let file_handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapAlloc cannot read the handle");
    let lpFileSizeHigh = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapAlloc cannot read the flags");

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
    emu.maps.write_qword(lpFileSizeHigh as u64, file_size);
}
