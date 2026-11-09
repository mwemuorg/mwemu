use crate::emu;

pub fn WSCUpdateProvider32(emu: &mut emu::Emu) {
    let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSCUpdateProvider32 ptr:0x{:x}", err_ptr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
