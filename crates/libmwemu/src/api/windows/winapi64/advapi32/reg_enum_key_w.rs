use crate::emu;

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!RegEnumKeyW hkey:0x{:x} index:{}",
        hkey,
        index
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    for _ in 0..8 {}
}
