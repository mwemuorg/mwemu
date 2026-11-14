use crate::emu;

pub fn ProcessPrng(emu: &mut emu::Emu) {
    let pb_data = emu.regs().rcx;
    let cb_data = emu.regs().rdx;
    log_red!(
        emu,
        "bcrypt!ProcessPrng buf: 0x{:x} len: {}",
        pb_data,
        cb_data
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for i in 0..cb_data {
        let byte = ((emu.pos.wrapping_add(i)) & 0xFF) as u8;
        emu.maps.write_byte(pb_data + i, byte);
    }
    emu.regs_mut().rax = 1; // BOOL TRUE
}
