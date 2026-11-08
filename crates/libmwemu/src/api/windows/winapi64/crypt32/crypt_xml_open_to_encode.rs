use crate::emu;
use crate::winapi::helper;

pub fn CryptXmlOpenToEncode(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let hndl = helper::handler_create("CryptXmlOpenToEncode");
    log_red!(
        emu,
        "crypt32!CryptXmlOpenToEncode arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
        arg1,
        arg2,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
