use crate::emu;
use crate::winapi::helper;

pub fn CryptXmlImportPublicKey(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(emu, "crypt32!CryptXmlImportPublicKey arg1:0x{:x}", arg1);
    let handle = helper::handler_create("CryptXmlImportPublicKey");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
