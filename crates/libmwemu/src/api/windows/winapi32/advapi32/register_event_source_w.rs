use crate::emu;
use crate::winapi::helper;

pub fn RegisterEventSourceW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegisterEventSourceW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("advapi32://RegisterEventSourceW");
    emu.regs_mut().rax = hndl;
}
