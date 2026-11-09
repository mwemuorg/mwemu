use crate::emu;

pub fn IsWellKnownSid(emu: &mut emu::Emu) {
    let sid = emu.regs().rcx;
    let well_known_type = emu.regs().rdx as u32;
    let result = if sid != 0 {
        let revision = emu.maps.read_byte(sid).unwrap_or(0);
        let sub_count = emu.maps.read_byte(sid + 1).unwrap_or(0);
        if revision == 1 && sub_count > 0 {
            let sub0 = emu.maps.read_dword(sid + 8).unwrap_or(0);
            match well_known_type {
                0 => sub0 == 0,
                1 => sub0 == 0,
                22 => sub0 == 544,
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };
    log_red!(
        emu,
        "kernelbase!IsWellKnownSid sid:0x{:x} type:{} => {}",
        sid,
        well_known_type,
        result
    );
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
    emu.regs_mut().rax = if result { 1 } else { 0 };
}
