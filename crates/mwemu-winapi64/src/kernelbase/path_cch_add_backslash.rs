use crate::emu;

pub fn PathCchAddBackslash(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let mut result_path = path.clone();
    if !result_path.ends_with('\\') && !result_path.is_empty() {
        result_path.push('\\');
    }
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(emu, "kernelbase!{} '{}' => '{}'", api, path, result_path);
    emu.regs_mut().rax = 0; // S_OK
}
