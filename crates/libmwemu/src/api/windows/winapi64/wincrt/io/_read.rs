use crate::emu;

pub fn _read(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    let buf = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    log_red!(
        emu,
        "wincrt!_read fd:{} buf:0x{:x} count:{}",
        fd,
        buf,
        count
    );
    if buf != 0 && count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
