use crate::emu;
use crate::winapi::helper;

pub fn CryptSIPRemoveSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPRemoveSignedDataMsg p_subject_info:0x{:x} dw_index:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        dw_index,
        dw_reserved
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
