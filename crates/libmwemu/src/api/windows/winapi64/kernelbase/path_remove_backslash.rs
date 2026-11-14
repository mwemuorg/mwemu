use crate::emu;
use crate::winapi::common::strings;

pub fn PathRemoveBackslashA(emu: &mut emu::Emu) {
    path_remove_backslash(emu, "PathRemoveBackslashA", false);
}

pub fn PathRemoveBackslashW(emu: &mut emu::Emu) {
    path_remove_backslash(emu, "PathRemoveBackslashW", true);
}

fn path_remove_backslash(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    if wide {
        if path.ends_with('\\') {
            let char_count = path.encode_utf16().count();
            let end_pos = (char_count - 1) * 2;
            emu.maps.write_word(path_ptr + end_pos as u64, 0);
            emu.regs_mut().rax = path_ptr + end_pos as u64;
        } else {
            let char_count = path.encode_utf16().count();
            emu.regs_mut().rax = path_ptr + (char_count * 2) as u64;
        }
    } else {
        if path.ends_with('\\') {
            let end_pos = path.len() - 1;
            emu.maps.write_byte(path_ptr + end_pos as u64, 0);
            emu.regs_mut().rax = path_ptr + end_pos as u64;
        } else {
            let end_pos = path.len();
            emu.regs_mut().rax = path_ptr + end_pos as u64;
        }
    }

    log_red!(emu, "kernelbase!{api} '{}'", path);
}
