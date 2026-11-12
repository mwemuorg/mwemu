use crate::emu;
use crate::winapi::helper;

pub fn _beginthreadex(emu: &mut emu::Emu) {
    let _security = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _stack_size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let start = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthreadex");
    emu.regs_mut().rax = hndl;
}
