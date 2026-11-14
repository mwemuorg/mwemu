use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateServiceEx(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let hndl = helper::handler_create("CreateServiceEx");
    log_red!(emu, "advapi32!CreateServiceEx => 0x{:x}", hndl);
    emu.regs_mut().rax = hndl;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
