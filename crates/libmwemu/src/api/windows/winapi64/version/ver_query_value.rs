use crate::emu;

pub(super) fn VerQueryValue(emu: &mut emu::Emu) {
    let _block = emu.regs().rcx;
    let subblock_ptr = emu.regs().rdx;
    let _buffer = emu.regs().r8;
    let _len = emu.regs().r9;

    let subblock = emu.maps.read_string(subblock_ptr);
    log_red!(emu, "version!VerQueryValue `{}`", subblock);

    emu.regs_mut().rax = 0;
}
