use crate::emu;

pub fn PathFindExtensionW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    if let Some(dot_pos) = filename.rfind('.') {
        let offset_in_filename =
            filename[..dot_pos].encode_utf16().count() + path.len() - filename.len();
        let ext_offset_chars = path[..path.len() - filename.len()].encode_utf16().count()
            + filename[..dot_pos].encode_utf16().count();
        emu.set_last_error(0);
        emu.regs_mut().rax = path_ptr + (ext_offset_chars as u64) * 2;
    } else {
        let total_chars: u64 = path.encode_utf16().count() as u64;
        emu.regs_mut().rax = path_ptr + total_chars * 2;
    }
    log_red!(emu, "kernelbase!PathFindExtensionW '{}'", path);
}
