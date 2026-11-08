use crate::emu;

pub fn CryptCATEnumerateCatAttr(emu: &mut emu::Emu) {
    let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATEnumerateCatAttr h_catalog:0x{:x} p_prev_attr:0x{:x}",
        h_catalog,
        p_prev_attr
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
