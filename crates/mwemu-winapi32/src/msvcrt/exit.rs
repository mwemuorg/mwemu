use crate::emu;

pub(super) fn exit(emu: &mut emu::Emu) {
    let status = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}
