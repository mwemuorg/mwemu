use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WahOpenHandleHelper(emu: &mut emu::Emu) {
    let helper_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WahOpenHandleHelper ptr:0x{:x}", helper_ptr);
    let handle = helper::handler_create("wah_handle");
    if helper_ptr != 0 {
        emu.maps.write_dword(helper_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
