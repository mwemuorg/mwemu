use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptImportPublicKeyInfoEx2(emu: &mut emu::Emu) {
    let dw_cert_encoding = WinApiAbi::arg(emu, 0);
    let p_info = WinApiAbi::arg(emu, 1);
    let dw_flags = WinApiAbi::arg(emu, 2);
    let pv_aux_info = WinApiAbi::arg(emu, 3);
    let ph_key = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptImportPublicKeyInfoEx2 0x{:x}",
        dw_cert_encoding
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
