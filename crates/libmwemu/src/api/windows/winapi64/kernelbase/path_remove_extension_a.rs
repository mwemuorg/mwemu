use crate::emu;

pub fn PathRemoveExtensionA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    if path_ptr != 0 {
        let path = emu.maps.read_string(path_ptr);
        let filename = path
            .rfind('\\')
            .or_else(|| path.rfind('/'))
            .map(|p| &path[p + 1..])
            .unwrap_or(&path);
        if let Some(dot_pos) = filename.rfind('.') {
            let abs_dot = path.len() - filename.len() + dot_pos;
            emu.maps.write_byte(path_ptr + abs_dot as u64, 0);
        }
        log_red!(emu, "kernelbase!PathRemoveExtensionA '{}'", path);
    }
}
