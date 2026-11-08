use crate::emu;
use crate::winapi::helper;

pub fn PFXImportCertStore(emu: &mut emu::Emu) {
    let pfx_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _password_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
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
