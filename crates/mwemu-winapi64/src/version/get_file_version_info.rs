use crate::emu;

pub(super) fn GetFileVersionInfo(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let _handle = emu.regs().rdx;
    let _len = emu.regs().r8;
    let _data = emu.regs().r9;

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfo `{}`", filename);

    emu.regs_mut().rax = 0;
}
