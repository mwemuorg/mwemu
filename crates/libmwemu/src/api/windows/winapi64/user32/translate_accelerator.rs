use crate::emu;

pub fn TranslateAccelerator(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    log_red!(
        emu,
        "user32!TranslateAccelerator p1:0x{:x} p2:0x{:x}",
        p1,
        p2
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
