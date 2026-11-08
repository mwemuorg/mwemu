use crate::emu;

pub fn CryptCATCDFEnumCatAttributes(emu: &mut emu::Emu) {
    let p_cdf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_prev_attr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumCatAttributes p_cdf:0x{:x} p_prev_attr:0x{:x} pfn_parse_error:0x{:x}",
        p_cdf,
        p_prev_attr,
        pfn_parse_error
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
