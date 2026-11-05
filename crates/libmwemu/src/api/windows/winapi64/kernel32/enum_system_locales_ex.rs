use crate::emu;

pub fn EnumSystemLocalesEx(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!EnumSystemLocalesEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
