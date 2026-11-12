use crate::emu;

pub fn fgets(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let max_count = emu.regs().rdx as i32;
    let stream = emu.regs().r8;
    log_red!(
        emu,
        "wincrt!fgets buf:0x{:x} max:{} stream:0x{:x}",
        buf,
        max_count,
        stream
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
