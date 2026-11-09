use crate::emu;
use crate::winapi::helper;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let dw_flags = emu.regs().rdx;
    log_red!(
        emu,
        "crypt32!CryptReleaseContext prov: 0x{:x} flags: 0x{:x}",
        h_prov,
        dw_flags
    );
    if h_prov != 0 {
        helper::handler_close(h_prov);
    }
    emu.regs_mut().rax = 1;
}
