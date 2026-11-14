use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptVerifyMessageSignature(emu: &mut emu::Emu) {
    let p_verify_para = WinApiAbi::arg(emu, 0);
    let dw_signer_index = WinApiAbi::arg(emu, 1);
    let pb_signed_blob = WinApiAbi::arg(emu, 2);
    let cb_signed_blob = WinApiAbi::arg(emu, 3);
    let ppb_decoded = WinApiAbi::arg(emu, 4);
    let pcb_decoded = WinApiAbi::arg(emu, 5);
    let pp_signer_cert = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "crypt32!CryptVerifyMessageSignature 0x{:x}",
        p_verify_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
