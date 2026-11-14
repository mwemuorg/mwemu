use crate::emu;
use crate::winapi::common::strings;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameA", false);
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameW", true);
}

fn get_long_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let short_ptr = emu.regs().rcx;
    let long_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let path = if short_ptr != 0 {
        strings::read_text(emu, short_ptr, wide)
    } else {
        String::new()
    };
    if wide {
        let char_count = path.encode_utf16().count() + 1;
        if long_ptr != 0 && buf_size > 0 {
            strings::write_string(emu, long_ptr, &path, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => len:{}", path, char_count);
        emu.set_last_error(0);
        emu.regs_mut().rax = char_count as u64;
    } else {
        if long_ptr != 0 && buf_size > 0 {
            strings::write_string(emu, long_ptr, &path, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => len:{}", path, path.len() + 1);
        emu.set_last_error(0);
        emu.regs_mut().rax = (path.len() + 1) as u64;
    }
}
