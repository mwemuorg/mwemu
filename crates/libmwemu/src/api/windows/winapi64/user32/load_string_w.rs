use crate::emu;

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    let _hinst = emu.regs().rcx;
    let id = emu.regs().rdx;
    let buf = emu.regs().r8;
    let buf_max = emu.regs().r9 as usize;
    log_red!(emu, "user32!LoadStringW id:{} buf:0x{:x}", id, buf);
    if buf != 0 && buf_max > 0 {
        emu.maps.write_word(buf, 0);
    }
    emu.regs_mut().rax = 0;
}
