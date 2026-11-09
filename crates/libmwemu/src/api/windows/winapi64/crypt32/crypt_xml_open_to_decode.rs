use crate::emu;
use crate::winapi::helper;

pub fn CryptXmlOpenToDecode(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let hndl = helper::handler_create("CryptXmlOpenToDecode");
    log_red!(
        emu,
        "crypt32!CryptXmlOpenToDecode arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
        arg1,
        arg2,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
