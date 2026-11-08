use crate::emu;

pub fn CryptSIPVerifyIndirectData(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_indirect_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPVerifyIndirectData p_subject_info:0x{:x} p_indirect_data:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        p_indirect_data,
        dw_reserved
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
