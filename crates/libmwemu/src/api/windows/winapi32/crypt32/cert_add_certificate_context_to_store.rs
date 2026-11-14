use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertAddCertificateContextToStore(emu: &mut emu::Emu) {
    let store = WinApiAbi::arg(emu, 0);
    let cert_ctx = WinApiAbi::arg(emu, 1);
    let add_disp = WinApiAbi::arg(emu, 2);
    let out_ptr = WinApiAbi::arg(emu, 3);
    let hndl = helper::handler_create("cert_context");
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "crypt32!CertAddCertificateContextToStore store:0x{:x} disp:{} => 0x{:x}",
        store,
        add_disp,
        hndl
    );
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
