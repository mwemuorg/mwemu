use crate::emu;
use crate::winapi::common::strings;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesA", false);
}

fn set_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let attrs = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' attrs:0x{:x}", path, attrs);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesW", true);
}
