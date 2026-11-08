use crate::emu;

pub fn StretchBlt(emu: &mut emu::Emu) {
    let hdc_dest = emu.regs().rcx;
    let x_dest = emu.regs().rdx as i32;
    let y_dest = emu.regs().r8 as i32;
    let w_dest = emu.regs().r9 as i32;
    let h_dest = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let hdc_src = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let x_src = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let y_src = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let w_src = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    let h_src = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as i32;
    let rop = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0) as u32;
    log_red!(
        emu,
        "gdi32!StretchBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} {}x{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        w_dest,
        h_dest,
        hdc_src,
        x_src,
        y_src,
        w_src,
        h_src,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
