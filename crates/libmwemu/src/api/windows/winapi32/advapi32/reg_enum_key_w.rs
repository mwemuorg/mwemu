use crate::emu;

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!RegEnumKeyW hkey:0x{:x} index:{}",
        hkey,
        index
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
