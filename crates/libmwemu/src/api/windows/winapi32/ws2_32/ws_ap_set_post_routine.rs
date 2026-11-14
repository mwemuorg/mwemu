use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSApSetPostRoutine(emu: &mut emu::Emu) {
    let routine_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSApSetPostRoutine routine:0x{:x}", routine_ptr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
