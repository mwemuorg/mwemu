use crate::emu;

pub fn GetDlgItemTextA(emu: &mut emu::Emu) {
    let _hdlg = emu.regs().rcx;
    let _id = emu.regs().rdx as u32;
    let buf = emu.regs().r8;
    let max_count = emu.regs().r9 as u32;
    log_red!(emu, "user32!GetDlgItemTextA id:{} buf:0x{:x}", _id, buf);
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf != 0 && max_count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.regs_mut().rax = 0;
}
