use crate::emu;

pub fn RegEnumValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!RegEnumValueA hkey:0x{:x} idx:{}",
        hkey,
        index
    );
    for _ in 0..8 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}
