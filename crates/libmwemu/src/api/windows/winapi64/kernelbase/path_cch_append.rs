use crate::emu;

pub fn PathCchAppend(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let more_ptr = emu.regs().r8;
    let path = emu.maps.read_wide_string(path_ptr);
    let more = emu.maps.read_wide_string(more_ptr);
    let result_path = if more.starts_with('\\') || (more.len() >= 2 && more.as_bytes()[1] == b':') {
        more.clone()
    } else if path.ends_with('\\') {
        format!("{}{}", path, more)
    } else {
        format!("{}\\{}", path, more)
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!{} '{}' + '{}' => '{}'",
        api,
        path,
        more,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
