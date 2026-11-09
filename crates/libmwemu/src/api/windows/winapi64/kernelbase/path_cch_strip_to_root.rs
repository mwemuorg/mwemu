use crate::emu;

pub fn PathCchStripToRoot(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let result_path =
        if path.len() >= 3 && path.as_bytes()[1] == b':' && path.as_bytes()[2] == b'\\' {
            path[..3].to_string()
        } else if path.starts_with('\\') {
            "\\".to_string()
        } else {
            path.clone()
        };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchStripToRoot '{}' => '{}'",
        path,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
