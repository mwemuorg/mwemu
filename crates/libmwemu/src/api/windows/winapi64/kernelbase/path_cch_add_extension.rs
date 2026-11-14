use crate::emu;

pub fn PathCchAddExtension(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let ext_ptr = emu.regs().r8;
    let path = emu.maps.read_wide_string(path_ptr);
    let ext = emu.maps.read_wide_string(ext_ptr);
    let result_path = if path.contains('.') {
        path.clone()
    } else {
        let dot_ext = if ext.starts_with('.') {
            ext.clone()
        } else {
            format!(".{}", ext)
        };
        format!("{}{}", path, dot_ext)
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchAddExtension '{}' + '{}' => '{}'",
        path,
        ext,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
