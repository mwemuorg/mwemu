use crate::emu;

pub fn GetSidSubAuthorityCount(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetSidSubAuthorityCount sid:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        let _revision = emu.maps.read_byte(p0).unwrap_or(0);
        let count = emu.maps.read_byte(p0 + 1).unwrap_or(1);
        emu.regs_mut().rax = p0 + 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
