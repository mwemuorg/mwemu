use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn LoadCursorA(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let name = arg(emu, 1);
    log_red!(
        emu,
        "user32!LoadCursorA hInstance: 0x{:x} name: 0x{:x}",
        hinstance,
        name
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
