use crate::emu;

pub fn RegLoadMUIStringA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!RegLoadMUIStringA '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn RegLoadMUIStringW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!RegLoadMUIStringW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
