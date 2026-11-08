use crate::emu;
use crate::winapi::helper;

pub fn EventRegister(emu: &mut emu::Emu) {
    let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let enable_callback = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let callback_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let reg_handle = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
