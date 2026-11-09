use crate::emu;
use crate::winapi::helper;

pub fn GetLengthSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    let sub_count = if sid_ptr != 0 {
        emu.maps.read_byte(sid_ptr + 1).unwrap_or(5) as u64
    } else {
        5
    };
    let len = 8 + sub_count * 4;
    log_red!(emu, "advapi32!GetLengthSid sid:0x{:x} => {}", sid_ptr, len);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = len;
}
