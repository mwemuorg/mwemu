use crate::emu;
use crate::winapi::helper;

pub fn CertDuplicateStore(emu: &mut emu::Emu) {
    let context = emu.regs().rcx;
    log_red!(emu, "crypt32!CertDuplicateStore ctx:0x{:x}", context);
    emu.set_last_error(0);
    let handle = helper::handler_create("certduplicatestore");
    emu.regs_mut().rax = handle;
}
