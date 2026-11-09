use crate::emu;
use crate::winapi::helper;

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
