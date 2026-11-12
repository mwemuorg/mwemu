use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn FreeEncryptionCertificateHashList(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    helper::handler_close(handle);
    log_red!(
        emu,
        "advapi32!FreeEncryptionCertificateHashList handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
