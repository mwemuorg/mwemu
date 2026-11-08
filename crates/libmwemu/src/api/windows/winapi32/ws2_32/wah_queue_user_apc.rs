use crate::emu;
use crate::winapi::helper;

pub fn WahQueueUserApc(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WahQueueUserApc arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("WahQueueUserApc");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
