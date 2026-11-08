use crate::emu;
use crate::winapi::helper;

pub fn CryptCATCDFClose(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "crypt32!CryptCATCDFClose handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}
