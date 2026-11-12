use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn EventRegister(emu: &mut emu::Emu) {
    let provider_id = WinApiAbi::arg(emu, 0) as u32;
    let enable_callback = WinApiAbi::arg(emu, 1) as u32;
    let callback_ctx = WinApiAbi::arg(emu, 2) as u32;
    let reg_handle = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "advapi32!EventRegister prov: 0x{:x} cb: 0x{:x} ctx: 0x{:x} out: 0x{:x}",
        provider_id,
        enable_callback,
        callback_ctx,
        reg_handle
    );
    if reg_handle != 0 {
        let h = helper::handler_create("event_provider");
        emu.maps.write_dword(reg_handle as u64, h as u32);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
