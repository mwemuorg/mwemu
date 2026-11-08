use crate::emu;

pub(super) fn GetFileVersionInfoSize(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let handle_ptr = emu.regs().rdx;

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfoSize `{}`", filename);

    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
