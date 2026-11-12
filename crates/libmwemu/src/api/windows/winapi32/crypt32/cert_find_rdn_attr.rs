use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertFindRDNAttr(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertFindRDNAttr arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = helper::handler_create("CertFindRDNAttr");
}
