use crate::emu;
use crate::winapi::helper;

pub fn SHAssocEnumHandlers(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "shell32!SHAssocEnumHandlers arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shassocenumhandlers");
    emu.regs_mut().rax = handle;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
