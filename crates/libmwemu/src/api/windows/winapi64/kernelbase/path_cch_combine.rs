use crate::emu;

pub fn PathCchCombine(emu: &mut emu::Emu, api: &str) {
    let out_ptr = emu.regs().rcx;
    let cch_out = emu.regs().rdx as usize;
    let dir_ptr = emu.regs().r8;
    let file_ptr = emu.regs().r9;
    let dir = emu.maps.read_wide_string(dir_ptr);
    let file = emu.maps.read_wide_string(file_ptr);
    let result_path = if file.starts_with('\\') || (file.len() >= 2 && file.as_bytes()[1] == b':') {
        file.clone()
    } else if dir.ends_with('\\') {
        format!("{}{}", dir, file)
    } else {
        format!("{}\\{}", dir, file)
    };
    emu.maps.write_wide_string(out_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!{} '{}' + '{}' => '{}'",
        api,
        dir,
        file,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
