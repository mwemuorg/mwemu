use crate::emu;

pub fn PathCchStripPrefix(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let result_path = if let Some(stripped) = path.strip_prefix("\\\\?\\") {
        stripped.to_string()
    } else {
        path.clone()
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchStripPrefix '{}' => '{}'",
        path,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
