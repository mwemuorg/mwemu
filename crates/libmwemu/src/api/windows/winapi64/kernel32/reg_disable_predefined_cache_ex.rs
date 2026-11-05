use crate::emu;

pub fn RegDisablePredefinedCacheEx(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!RegDisablePredefinedCacheEx hkey:0x{:x} param2:0x{:x} param3:0x{:x}",
        hkey,
        param2,
        param3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
