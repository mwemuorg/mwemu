use crate::emu;

pub fn RegEnumValueA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegEnumValueA hkey:0x{:x} idx:{}",
        hkey,
        index
    );
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}
