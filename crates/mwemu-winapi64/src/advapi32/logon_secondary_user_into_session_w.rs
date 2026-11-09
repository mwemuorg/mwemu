use crate::emu;

pub fn LogonSecondaryUserIntoSessionW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let p5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!LogonSecondaryUserIntoSessionW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
