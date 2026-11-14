use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn GetAddrInfoExCancel(emu: &mut emu::Emu) {
    let cancel_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!GetAddrInfoExCancel ptr:0x{:x}", cancel_ptr);
    let handle = helper::handler_create("GetAddrInfoExCancel");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
