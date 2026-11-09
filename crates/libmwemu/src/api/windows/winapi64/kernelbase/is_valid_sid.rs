use crate::emu;

pub fn IsValidSid(emu: &mut emu::Emu) {
    let p_sid = emu.regs().rcx;
    let valid = if p_sid != 0 {
        let revision = emu.maps.read_byte(p_sid).unwrap_or(0);
        let sub_count = emu.maps.read_byte(p_sid + 1).unwrap_or(0);
        if revision == 1 && sub_count <= 15 {
            1u64
        } else {
            0
        }
    } else {
        0
    };
    log_red!(emu, "kernelbase!IsValidSid sid:0x{:x} => {}", p_sid, valid);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
    emu.regs_mut().rax = valid;
}
