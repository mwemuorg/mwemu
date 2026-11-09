use crate::emu;
use crate::winapi::helper;

pub fn _beginthread(emu: &mut emu::Emu) {
    let start = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthread");
    emu.regs_mut().rax = hndl;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
