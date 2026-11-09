use crate::emu;

pub fn CryptCATCDFEnumAttributesWithCDFTag(emu: &mut emu::Emu) {
    let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pwsz_member_tag = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_member = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumAttributesWithCDFTag 0x{:x}",
        p_cdf
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
