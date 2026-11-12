use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetCompressedFileSizeA(emu: &mut emu::Emu) {
    get_compressed_file_size(emu, "GetCompressedFileSizeA", false);
}

fn get_compressed_file_size(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}' p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 4096;
}

pub fn GetCompressedFileSizeW(emu: &mut emu::Emu) {
    get_compressed_file_size(emu, "GetCompressedFileSizeW", true);
}
