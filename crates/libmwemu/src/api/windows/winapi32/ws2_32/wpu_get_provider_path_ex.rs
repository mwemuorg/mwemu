use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WPUGetProviderPathEx(emu: &mut emu::Emu) {
    let provider_id_ptr = WinApiAbi::arg(emu, 0);
    let path_ptr = WinApiAbi::arg(emu, 1);
    let path_len_ptr = WinApiAbi::arg(emu, 2);
    let flags = WinApiAbi::arg(emu, 3) as u32;
    log_red!(emu, "ws2_32!WPUGetProviderPathEx flags:{}", flags);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
