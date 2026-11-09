use crate::emu;
use crate::winapi::helper;

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store);
    emu.regs_mut().rax = 1;
}
