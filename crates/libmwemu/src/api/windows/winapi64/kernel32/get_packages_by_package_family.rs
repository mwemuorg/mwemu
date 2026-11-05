use crate::emu;

pub fn GetPackagesByPackageFamily(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!GetPackagesByPackageFamily p1:0x{:x} p2:0x{:x} p3:0x{:x} p4:0x{:x}",
        param1,
        param2,
        param3,
        param4
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
