use crate::emu;

pub fn CryptFindOIDInfo(emu: &mut emu::Emu) {
    let dw_key_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pv_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_group_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptFindOIDInfo dw_key_type:0x{:x} pv_key:0x{:x} dw_group_id:0x{:x}",
        dw_key_type,
        pv_key,
        dw_group_id
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
