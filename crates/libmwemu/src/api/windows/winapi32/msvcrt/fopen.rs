use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn fopen(emu: &mut emu::Emu) {
    let filepath_ptr = WinApiAbi::arg(emu, 0);
    let mode_ptr = WinApiAbi::arg(emu, 1);

    if filepath_ptr == 0 || mode_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!fopen null ptrs");
        return;
    }

    let filepath = emu.maps.read_string(filepath_ptr);
    let mode = emu.maps.read_string(mode_ptr);

    log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);

    emu.regs_mut().rax = helper::handler_create(&filepath);
}
