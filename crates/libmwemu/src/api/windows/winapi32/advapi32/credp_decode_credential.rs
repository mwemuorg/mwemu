use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CredpDecodeCredential(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "advapi32!CredpDecodeCredential arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let src = WinApiAbi::arg(emu, 0);
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
