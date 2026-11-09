use crate::emu;

pub fn RegEnumValueW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegEnumValueW hkey:0x{:x} idx:{}",
        hkey,
        index
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}
