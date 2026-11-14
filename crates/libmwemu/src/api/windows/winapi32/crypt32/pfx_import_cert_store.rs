use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn PFXImportCertStore(emu: &mut emu::Emu) {
    let pfx_ptr = WinApiAbi::arg(emu, 0);
    let _password_ptr = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!PFXImportCertStore pfx: 0x{:x} flags: 0x{:x}",
        pfx_ptr,
        flags
    );
    emu.regs_mut().rax = helper::handler_create("certstore:pfx");
}
