use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = WinApiAbi::arg(emu, 0);
    let fptr = WinApiAbi::arg(emu, 1);

    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

    emu.regs_mut().rax = 0x2c2878;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
