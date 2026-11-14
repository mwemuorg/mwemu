use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptXmlGetTransforms(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CryptXmlGetTransforms arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
