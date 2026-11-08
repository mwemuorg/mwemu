use crate::emu;
use crate::winapi::helper;

pub fn CertRemoveStoreFromCollection(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertRemoveStoreFromCollection ctx:0x{:x}",
        cert_ctx
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
