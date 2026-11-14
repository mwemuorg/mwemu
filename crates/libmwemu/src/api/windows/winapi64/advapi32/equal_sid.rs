use crate::emu;

pub fn EqualSid(emu: &mut emu::Emu) {
    let sid1 = emu.regs().rcx;
    let sid2 = emu.regs().rdx;
    let equal = if sid1 != 0 && sid2 != 0 {
        let sub1 = emu.maps.read_byte(sid1 + 1).unwrap_or(0);
        let sub2 = emu.maps.read_byte(sid2 + 1).unwrap_or(0);
        if sub1 == sub2 {
            let len = (8 + sub1 as usize * 4).min(68);
            let b1 = emu.maps.read_bytes(sid1, len);
            let b2 = emu.maps.read_bytes(sid2, len);
            b1 == b2
        } else {
            false
        }
    } else {
        false
    };
    log_red!(
        emu,
        "advapi32!EqualSid 0x{:x} 0x{:x} => {}",
        sid1,
        sid2,
        equal
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if equal { 1 } else { 0 };
}
