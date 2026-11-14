use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertAddEncodedCertificateToStore(emu: &mut emu::Emu) {
    let store = WinApiAbi::arg(emu, 0);
    let encoding = WinApiAbi::arg(emu, 1);
    let cert_data = WinApiAbi::arg(emu, 2);
    let cert_len = WinApiAbi::arg(emu, 3);
    let add_disp = WinApiAbi::arg(emu, 4);
    let out_ptr = WinApiAbi::arg(emu, 5);
    let hndl = helper::handler_create("cert_encoded");
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "crypt32!CertAddEncodedCertificateToStore store:0x{:x} len:{}",
        store,
        cert_len
    );
    emu.regs_mut().rax = 1;
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}
