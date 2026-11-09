use crate::emu;
use crate::windows::constants;

pub fn BCryptGenRandom(emu: &mut emu::Emu) {
    let pb_buffer = emu.regs().rdx;
    let cb_buffer = emu.regs().r8;
    let flags = emu.regs().r9;

    log_red!(
        emu,
        "bcrypt!BCryptGenRandom buf: 0x{:x} len: {} flags: 0x{:x}",
        pb_buffer,
        cb_buffer,
        flags
    );

    for i in 0..cb_buffer {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_buffer + i, byte);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
