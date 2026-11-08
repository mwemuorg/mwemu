use crate::emu;

pub fn WSASetBlockingHook(emu: &mut emu::Emu) {
    let hook_fn = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
