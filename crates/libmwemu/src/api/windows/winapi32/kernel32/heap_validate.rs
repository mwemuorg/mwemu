use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn HeapValidate(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    log_red!(emu, "kernel32!HeapValidate p1:0x{:x} p2:0x{:x}", p1, p2);
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
