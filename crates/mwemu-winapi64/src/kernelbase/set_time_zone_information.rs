use crate::emu;

pub fn SetTimeZoneInformation(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!SetTimeZoneInformation ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        let _bias = emu.maps.read_dword(p0).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
