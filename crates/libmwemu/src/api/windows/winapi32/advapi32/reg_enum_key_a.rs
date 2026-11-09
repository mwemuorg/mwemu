use crate::emu;

pub fn RegEnumKeyA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!RegEnumKeyA hkey:0x{:x} index:{}",
        hkey,
        index
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
