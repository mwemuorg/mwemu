use crate::emu;
use crate::winapi::common::strings;

pub fn GetCompressedFileSizeA(emu: &mut emu::Emu) {
    get_compressed_file_size(emu, "GetCompressedFileSizeA", false);
}

fn get_compressed_file_size(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
