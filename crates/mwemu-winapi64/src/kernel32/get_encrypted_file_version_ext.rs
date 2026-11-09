use crate::emu;

pub fn GetEncryptedFileVersionExt(emu: &mut emu::Emu) {
    let file_name_ptr = emu.regs().rcx;
    let version_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetEncryptedFileVersionExt file:0x{:x} ver:0x{:x}",
        file_name_ptr,
        version_ptr
    );
    if version_ptr != 0 {
        emu.maps.write_dword(version_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
