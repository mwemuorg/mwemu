use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHFormatDrive(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!SHFormatDrive arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let src = WinApiAbi::arg(emu, 0);
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
