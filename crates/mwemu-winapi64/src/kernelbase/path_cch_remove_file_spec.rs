use crate::emu;

pub fn PathCchRemoveFileSpec(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let result_path = if let Some(pos) = path.rfind('\\') {
        path[..pos].to_string()
    } else {
        String::new()
    };
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(
        emu,
        "kernelbase!PathCchRemoveFileSpec '{}' => '{}'",
        path,
        result_path
    );
    emu.regs_mut().rax = 0; // S_OK
}
