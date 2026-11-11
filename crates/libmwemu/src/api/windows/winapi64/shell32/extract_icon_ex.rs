use crate::emu;
use crate::winapi::helper;

pub fn ExtractIconEx(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let icon_index = emu.regs().rdx as i32;
    let large_ptr = emu.regs().r8;
    let small_ptr = emu.regs().r9;
    let file = if file_ptr != 0 {
        emu.maps.read_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!ExtractIconEx file:'{}' index:{}",
        file,
        icon_index
    );
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_qword(large_ptr, handle);
    }
    if small_ptr != 0 {
        emu.maps.write_qword(small_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ExtractIconExA(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let icon_index = emu.regs().rdx as i32;
    let large_ptr = emu.regs().r8;
    let small_ptr = emu.regs().r9;
    let file = if file_ptr != 0 {
        emu.maps.read_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!ExtractIconExA file:'{}' index:{}",
        file,
        icon_index
    );
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_qword(large_ptr, handle);
    }
    if small_ptr != 0 {
        emu.maps.write_qword(small_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ExtractIconExW(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let icon_index = emu.regs().rdx as i32;
    let large_ptr = emu.regs().r8;
    let small_ptr = emu.regs().r9;
    let file = if file_ptr != 0 {
        emu.maps.read_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!ExtractIconExW file:'{}' index:{}",
        file,
        icon_index
    );
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_qword(large_ptr, handle);
    }
    if small_ptr != 0 {
        emu.maps.write_qword(small_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
