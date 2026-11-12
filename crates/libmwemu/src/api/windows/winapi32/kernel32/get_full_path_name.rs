use crate::emu;
use crate::winapi::common::strings;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let path = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let filename = strings::read_text(emu, file_ptr, wide);

    log_red!(emu, "kernel32!{api} file: {}", filename);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 10;
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}
