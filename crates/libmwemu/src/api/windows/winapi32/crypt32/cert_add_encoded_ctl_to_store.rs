use crate::emu;
use crate::winapi::helper;

pub fn CertAddEncodedCTLToStore(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertAddEncodedCTLToStore ctx:0x{:x}", cert_ctx);
    let handle = helper::handler_create("CertAddEncodedCTLToStore");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
