use crate::emu;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let codepage = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);

    log_red!(emu, "kernel32!IsValidCodePage {}", codepage);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
