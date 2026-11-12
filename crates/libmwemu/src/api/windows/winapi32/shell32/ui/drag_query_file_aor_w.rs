use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DragQueryFileAorW(emu: &mut emu::Emu) {
    let hdrop = WinApiAbi::arg(emu, 0);
    let index = WinApiAbi::arg(emu, 1) as u32;
    let buf_ptr = WinApiAbi::arg(emu, 2);
    let buf_size = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "shell32!DragQueryFileAorW hdrop:0x{:x} index:{}",
        hdrop,
        index
    );
    if index == 0xFFFFFFFF {
        emu.regs_mut().rax = 1;
    } else if buf_ptr != 0 && buf_size > 10 {
        emu.maps.write_string(buf_ptr, "C:\\file.txt");
        emu.regs_mut().rax = 10;
    } else {
        emu.regs_mut().rax = 10;
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
