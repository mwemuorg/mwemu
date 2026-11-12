use crate::emu;
use crate::winapi::common::strings;

pub fn DragQueryFile(emu: &mut emu::Emu) {
    let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "shell32!DragQueryFile hdrop:0x{:x} index:{}",
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

pub fn DragQueryFileA(emu: &mut emu::Emu) {
    drag_query_file(emu, "DragQueryFileA", false);
}

fn drag_query_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(emu, "shell32!{api} hdrop:0x{:x} index:{}", hdrop, index);
    if index == 0xFFFFFFFF {
        emu.regs_mut().rax = 1;
    } else if buf_ptr != 0 && buf_size > 10 {
        strings::write_string(emu, buf_ptr, "C:\\file.txt", wide);
        emu.regs_mut().rax = 10;
    } else {
        emu.regs_mut().rax = 10;
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}

pub fn DragQueryFileW(emu: &mut emu::Emu) {
    drag_query_file(emu, "DragQueryFileW", true);
}
