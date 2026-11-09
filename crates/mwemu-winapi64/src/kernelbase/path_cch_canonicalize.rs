use crate::emu;
use crate::winapi::helper;

pub fn PathCchCanonicalize(emu: &mut emu::Emu, api: &str) {
    let out_ptr = emu.regs().rcx;
    let cch_out = emu.regs().rdx as usize;
    let path_ptr = emu.regs().r8;
    let path = emu.maps.read_wide_string(path_ptr);
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('\\') {
        match part {
            "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    let result_path = parts.join("\\");
    emu.maps.write_wide_string(out_ptr, &result_path);
    log_red!(emu, "kernelbase!{} '{}' => '{}'", api, path, result_path);
    let _h = helper::handler_create("dir_entry_current");
    emu.regs_mut().rax = 0; // S_OK
}
