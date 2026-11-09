use crate::emu;
use crate::winapi::helper;

pub fn CryptMsgControl(emu: &mut emu::Emu) {
    let h_crypt_msg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_ctrl_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pv_ctrl_para = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptMsgControl 0x{:x}", h_crypt_msg);
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
