use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptMsgControl(emu: &mut emu::Emu) {
    let h_crypt_msg = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1);
    let dw_ctrl_type = WinApiAbi::arg(emu, 2);
    let pv_ctrl_para = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CryptMsgControl 0x{:x}", h_crypt_msg);
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
