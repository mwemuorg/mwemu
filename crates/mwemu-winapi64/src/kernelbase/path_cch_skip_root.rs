use crate::emu;

pub fn PathCchSkipRoot(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let result_ptr = emu.regs().rdx;
    let path = emu.maps.read_wide_string(path_ptr);
    let skip = if path.len() >= 3 && path.as_bytes()[1] == b':' && path.as_bytes()[2] == b'\\' {
        3
    } else if let Some(rest) = path.strip_prefix("\\\\") {
        if let Some(pos) = rest.find('\\') {
            pos + 3
        } else {
            path.len()
        }
    } else if path.starts_with('\\') {
        1
    } else {
        0
    };
    let after_root = path_ptr + (skip as u64) * 2;
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, after_root);
    }
    log_red!(emu, "kernelbase!PathCchSkipRoot '{}' skip:{}", path, skip);
    emu.regs_mut().rax = 0; // S_OK
}
