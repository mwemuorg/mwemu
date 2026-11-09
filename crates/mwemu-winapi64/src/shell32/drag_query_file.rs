use crate::emu;

pub fn DragQueryFile(emu: &mut emu::Emu) {
    let hdrop = emu.regs().rcx;
    let index = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9 as u32;
    log_red!(
        emu,
        "shell32!DragQueryFile hdrop:0x{:x} index:{}",
        hdrop,
        index
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if index == 0xFFFFFFFF {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else if buf_ptr != 0 && buf_size > 10 {
        emu.maps.write_string(buf_ptr, "C:\\file.txt");
        emu.regs_mut().rax = 10;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 10;
    }
}
