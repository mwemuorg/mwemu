use crate::emu;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let size = emu.regs().rdx as u32;
    let buff = emu.regs().r8;
    let file_part_ptr = emu.regs().r9;

    let filename = emu.maps.read_string(file_ptr);

    let full_path = if filename.contains('\\') || filename.contains('/') {
        filename.clone()
    } else {
        format!("{}\\{}", emu.cfg.cwd_path, filename)
    };

    log_red!(
        emu,
        "kernel32!GetFullPathNameA '{}' => '{}'",
        filename,
        full_path
    );

    let needed = full_path.len() as u32 + 1;

    if size == 0 || buff == 0 {
        emu.regs_mut().rax = needed as u64;
        return;
    }

    if size < needed {
        emu.regs_mut().rax = needed as u64;
        return;
    }

    emu.maps.write_string(buff, &full_path);

    if file_part_ptr != 0
        && let Some(pos) = full_path.rfind('\\')
    {
        let offset = pos as u64 + 1;
        emu.maps.write_qword(file_part_ptr, buff + offset);
    }

    emu.regs_mut().rax = (needed - 1) as u64;
}
