use crate::emu;

pub fn VerFindFileA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!VerFindFileA '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
