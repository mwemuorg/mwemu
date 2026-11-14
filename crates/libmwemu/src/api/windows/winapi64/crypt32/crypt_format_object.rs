use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptFormatObject(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    let arg7 = WinApiAbi::arg(emu, 6);
    let arg8 = WinApiAbi::arg(emu, 7);
    log_red!(emu, "crypt32!CryptFormatObject 0x{:x} 0x{:x}", arg1, arg2);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
