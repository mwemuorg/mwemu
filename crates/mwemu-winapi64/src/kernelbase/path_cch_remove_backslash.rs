use crate::emu;

pub fn PathCchRemoveBackslash(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let result_path = if path.len() > 1 && path.ends_with('\\') {
        path.trim_end_matches('\\').to_string()
    } else {
        path.clone()
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(emu, "kernelbase!{} '{}' => '{}'", api, path, result_path);
    emu.regs_mut().rax = 0; // S_OK
}
