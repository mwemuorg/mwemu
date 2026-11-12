use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EncodePointer(emu: &mut emu::Emu) {
    let ptr = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!EncodePointer ptr: 0x{:x}", ptr);

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr;
}
