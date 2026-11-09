use crate::emu;
use crate::winapi::helper;

pub fn CertAddEncodedCertificateToStore(emu: &mut emu::Emu) {
    let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cert_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cert_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let add_disp = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
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
