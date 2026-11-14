use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn StrCmpCA(emu: &mut emu::Emu) {
    let str1_ptr = WinApiAbi::arg(emu, 0);
    let str2_ptr = WinApiAbi::arg(emu, 1);

    stdcall_ret(emu, 2);

    if str1_ptr == 0 || str2_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!StrCmpA null ptrs");
        return;
    }

    let str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "msvcrt!StrCmpA {} == {}", str1, str2);

    if str1 == str2 {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = 0xffffffff;
    }
}
