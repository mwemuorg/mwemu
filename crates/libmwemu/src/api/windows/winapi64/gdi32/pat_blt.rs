use crate::emu;

pub fn PatBlt(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let w = emu.regs().r9 as i32;
    let h = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let rop = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    log_red!(
        emu,
        "gdi32!PatBlt hdc:0x{:x} x:{} y:{} w:{} h:{} rop:0x{:x}",
        hdc,
        x,
        y,
        w,
        h,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
