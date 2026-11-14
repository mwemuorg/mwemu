use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let filename_ptr = WinApiAbi::arg(emu, 0);
    let filename = strings::read_text(emu, filename_ptr, wide);
    log_red!(emu, "kernel32!{api} file: {}", filename);
    if wide {
        emu.stack_pop32(false);
        emu.regs_mut().rax = 0x123;
    } else {
        emu.regs_mut().rax = 0x123;
        emu.stack_pop32(false);
    }
}
