use crate::emu;

pub fn PathRemoveExtensionA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    if path_ptr != 0 {
        let path = emu.maps.read_string(path_ptr);
        let filename = path
            .rfind('\\')
            .or_else(|| path.rfind('/'))
            .map(|p| &path[p + 1..])
            .unwrap_or(&path);
        if let Some(dot_pos) = filename.rfind('.') {
            let abs_dot = path.len() - filename.len() + dot_pos;
            emu.maps.write_byte(path_ptr + abs_dot as u64, 0);
        }
        log_red!(emu, "kernelbase!PathRemoveExtensionA '{}'", path);
    }
}

pub fn PathRemoveExtensionW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    if path_ptr != 0 {
        let path = emu.maps.read_wide_string(path_ptr);
        let filename = path
            .rfind('\\')
            .or_else(|| path.rfind('/'))
            .map(|p| &path[p + 1..])
            .unwrap_or(&path);
        if let Some(dot_pos) = filename.rfind('.') {
            let prefix_chars = path[..path.len() - filename.len()].encode_utf16().count();
            let dot_chars = filename[..dot_pos].encode_utf16().count();
            let abs_char_pos = prefix_chars + dot_chars;
            emu.maps.write_word(path_ptr + (abs_char_pos as u64) * 2, 0);
        }
        log_red!(emu, "kernelbase!PathRemoveExtensionW '{}'", path);
    }
}
