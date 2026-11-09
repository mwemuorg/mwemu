use crate::emu;
use crate::winapi::helper;

pub fn WahOpenCurrentThread(emu: &mut emu::Emu) {
    let helper_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WahOpenCurrentThread ptr:0x{:x}", helper_ptr);
    let handle = helper::handler_create("wah_handle");
    if helper_ptr != 0 {
        emu.maps.write_dword(helper_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
