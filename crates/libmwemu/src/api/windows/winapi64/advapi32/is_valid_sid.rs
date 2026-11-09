use crate::emu;
use crate::winapi::helper;

pub fn IsValidSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    let valid = if sid_ptr != 0 {
        let revision = emu.maps.read_byte(sid_ptr).unwrap_or(0);
        let sub_count = emu.maps.read_byte(sid_ptr + 1).unwrap_or(0);
        revision == 1 && sub_count <= 15
    } else {
        false
    };
    log_red!(emu, "advapi32!IsValidSid sid:0x{:x} => {}", sid_ptr, valid);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = if valid { 1 } else { 0 };
}
