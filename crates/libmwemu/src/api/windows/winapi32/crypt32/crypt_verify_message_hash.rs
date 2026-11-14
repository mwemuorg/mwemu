use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptVerifyMessageHash(emu: &mut emu::Emu) {
    let p_hash_para = WinApiAbi::arg(emu, 0);
    let pb_hashed_blob = WinApiAbi::arg(emu, 1);
    let cb_hashed_blob = WinApiAbi::arg(emu, 2);
    let pb_to_be_hashed = WinApiAbi::arg(emu, 3);
    let pcb_to_be_hashed = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptVerifyMessageHash 0x{:x}", p_hash_para);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
