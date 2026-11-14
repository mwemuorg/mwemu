use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let _padding_info = emu.regs().rdx;
    let pb_hash = emu.regs().r8;
    let cb_hash = emu.regs().r9 as u32;
    let pb_signature = WinApiAbi::arg(emu, 4);
    let cb_signature = WinApiAbi::arg(emu, 5) as u32;
    let _flags = WinApiAbi::arg(emu, 6) as u32;

    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) => valid",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}
