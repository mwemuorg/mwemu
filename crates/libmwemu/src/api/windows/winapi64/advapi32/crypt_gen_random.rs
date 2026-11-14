use crate::emu;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let dw_len = emu.regs().rdx;
    let pb_buffer = emu.regs().r8;

    log_red!(
        emu,
        "advapi32!CryptGenRandom len={} buf=0x{:x}",
        dw_len,
        pb_buffer
    );

    if pb_buffer != 0 && dw_len > 0 {
        let tick = emu.tick as u8;
        for i in 0..dw_len {
            let byte = tick.wrapping_add(i as u8);
            emu.maps.write_byte(pb_buffer + i, byte);
        }
    }

    emu.regs_mut().rax = 1;
}
