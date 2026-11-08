use crate::emu;
use crate::winapi::helper;

pub fn CryptImportPublicKeyInfo(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CryptImportPublicKeyInfo 0x{:x} 0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("CryptImportPublicKeyInfo");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
