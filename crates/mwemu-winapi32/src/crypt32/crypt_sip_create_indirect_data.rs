use crate::emu;

pub fn CryptSIPCreateIndirectData(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pcb_indirect_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_indirect_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPCreateIndirectData p_subject_info:0x{:x} pcb_indirect_data:0x{:x} p_indirect_data:0x{:x}",
        p_subject_info,
        pcb_indirect_data,
        p_indirect_data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
