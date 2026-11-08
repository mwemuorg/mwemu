use crate::emu;

pub fn CryptCATGetMemberInfo(emu: &mut emu::Emu) {
    let h_catalog = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pwsz_ref_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_cat_member = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATGetMemberInfo h_catalog:0x{:x} pwsz_ref_tag:0x{:x} p_cat_member:0x{:x}",
        h_catalog,
        pwsz_ref_tag,
        p_cat_member
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
