use crate::emu;
use crate::winapi::helper;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!TlsAlloc");
    let _h = helper::handler_create("tls_slot");

    emu.tls64_mut().push(0);
    emu.set_last_error(0);
    emu.regs_mut().rax = (emu.tls64().len() - 1) as u64; // Return index of newly allocated slot
}
