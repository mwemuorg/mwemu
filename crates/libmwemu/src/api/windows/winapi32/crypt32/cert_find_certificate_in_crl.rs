use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertFindCertificateInCRL(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let arg3 = WinApiAbi::arg(emu, 2);
    let arg4 = WinApiAbi::arg(emu, 3);
    let arg5 = WinApiAbi::arg(emu, 4);
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertFindCertificateInCRL arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5
    );
    emu.regs_mut().rax = helper::handler_create("CertFindCertificateInCRL");
}
