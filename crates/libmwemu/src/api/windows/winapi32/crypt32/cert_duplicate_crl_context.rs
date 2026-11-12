use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertDuplicateCRLContext(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "crypt32!CertDuplicateCRLContext arg1:0x{:x}", arg1);
    emu.regs_mut().rax = helper::handler_create("CertDuplicateCRLContext");
}
