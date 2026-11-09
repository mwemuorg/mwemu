use crate::emu;

pub fn CryptCATCatalogInfoFromContext(emu: &mut emu::Emu) {
    let h_cat_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_cat_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATCatalogInfoFromContext h_cat_info:0x{:x} p_cat_info:0x{:x} dw_flags:0x{:x}",
        h_cat_info,
        p_cat_info,
        dw_flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
