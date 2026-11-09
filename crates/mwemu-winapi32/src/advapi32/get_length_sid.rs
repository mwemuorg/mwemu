use crate::emu;

pub fn GetLengthSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sub_count = if sid_ptr != 0 {
        emu.maps.read_byte(sid_ptr + 1).unwrap_or(1) as u64
    } else {
        1
    };
    let len = 8 + sub_count * 4;
    log_red!(emu, "advapi32!GetLengthSid 0x{:x} => {}", sid_ptr, len);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = len;
}
