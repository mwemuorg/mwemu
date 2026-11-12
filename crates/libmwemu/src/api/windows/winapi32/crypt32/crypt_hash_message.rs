use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptHashMessage(emu: &mut emu::Emu) {
    let p_hash_para = WinApiAbi::arg(emu, 0);
    let f_detached_hash = WinApiAbi::arg(emu, 1);
    let c_to_be_hashed = WinApiAbi::arg(emu, 2);
    let rgb_pb_to_be_hashed = WinApiAbi::arg(emu, 3);
    let rgcb_to_be_hashed = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptHashMessage 0x{:x}", p_hash_para);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}
