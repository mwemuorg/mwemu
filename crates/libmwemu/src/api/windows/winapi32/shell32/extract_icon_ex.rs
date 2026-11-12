use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn ExtractIconEx(emu: &mut emu::Emu) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let icon_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
    let large_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let small_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
        emu.maps.write_dword(large_ptr, handle as u32);
    }
    if small_ptr != 0 {
        emu.maps.write_dword(small_ptr, handle as u32);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}

pub fn ExtractIconExA(emu: &mut emu::Emu) {
    extract_icon_ex(emu, "ExtractIconExA", false);
}

fn extract_icon_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let icon_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
    let large_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let small_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let file = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "shell32!{api} file:'{}' index:{}", file, icon_index);
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_dword(large_ptr, handle as u32);
    }
    if small_ptr != 0 {
        emu.maps.write_dword(small_ptr, handle as u32);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn ExtractIconExW(emu: &mut emu::Emu) {
    extract_icon_ex(emu, "ExtractIconExW", true);
}
