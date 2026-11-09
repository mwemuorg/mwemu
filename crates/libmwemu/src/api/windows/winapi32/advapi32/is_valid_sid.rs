use crate::emu;

pub fn IsValidSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let valid = if sid_ptr != 0 {
        let revision = emu.maps.read_byte(sid_ptr).unwrap_or(0);
        let sub_count = emu.maps.read_byte(sid_ptr + 1).unwrap_or(0);
        revision == 1 && sub_count <= 15
    } else {
        false
    };
    log_red!(
        emu,
        "advapi32!IsValidSid 0x{:x} => {}",
        sid_ptr,
        valid as u32
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = valid as u64;
}
