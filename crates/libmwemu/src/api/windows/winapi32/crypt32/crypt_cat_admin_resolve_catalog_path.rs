use crate::emu;

pub fn CryptCATAdminResolveCatalogPath(emu: &mut emu::Emu) {
    let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pwsz_catalog_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_resolve_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATAdminResolveCatalogPath h_cat_admin:0x{:x} pwsz_catalog_file:0x{:x} p_resolve_info:0x{:x}",
        h_cat_admin,
        pwsz_catalog_file,
        p_resolve_info
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
