use crate::emu;

pub fn PathCchFindExtension(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let cch_path = emu.regs().rdx as usize;
    let ext_out = emu.regs().r8;
    let path = emu.maps.read_wide_string(path_ptr);
    let ext_offset = if let Some(pos) = path.rfind('.') {
        if path[pos..].contains('\\') {
            path.len()
        } else {
            pos
        }
    } else {
        path.len()
    };
    let result_addr = path_ptr + (ext_offset as u64) * 2;
    if ext_out != 0 {
        emu.maps.write_qword(ext_out, result_addr);
    }
    log_red!(
        emu,
        "kernelbase!PathCchFindExtension '{}' ext_offset:{}",
        path,
        ext_offset
    );
    emu.regs_mut().rax = 0; // S_OK
}
