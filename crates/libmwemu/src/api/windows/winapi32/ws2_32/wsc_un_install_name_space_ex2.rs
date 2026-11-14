use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSCUnInstallNameSpaceEx2(emu: &mut emu::Emu) {
    let err_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSCUnInstallNameSpaceEx2 ptr:0x{:x}", err_ptr);
    let handle = helper::handler_create("WSCUnInstallNameSpaceEx2");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
