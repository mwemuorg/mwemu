use crate::emu;
use crate::winapi::helper;

pub fn CertAddCertificateContextToStore(emu: &mut emu::Emu) {
    let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let add_disp = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
