use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertCreateCRLContext(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let arg3 = WinApiAbi::arg(emu, 2);
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertCreateCRLContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.regs_mut().rax = helper::handler_create("CertCreateCRLContext");
}
