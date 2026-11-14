use crate::emu;
use crate::winapi::common::strings;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = emu.regs().rcx;
    let size = emu.regs().rdx as u32;
    let buff = emu.regs().r8;
    let file_part_ptr = emu.regs().r9;
    let filename = strings::read_text(emu, file_ptr, wide);
    let full_path = if filename.contains('\\') || filename.contains('/') {
        filename.clone()
    } else {
        format!("{}\\{}", emu.cfg.cwd_path, filename)
    };
    log_red!(emu, "kernel32!{api} '{}' => '{}'", filename, full_path);
    if wide {
        let needed = full_path.chars().count() as u32 + 1;
        if size == 0 || buff == 0 {
            emu.regs_mut().rax = needed as u64;
            return;
        }
        if size < needed {
            emu.regs_mut().rax = needed as u64;
            return;
        }
        strings::write_string(emu, buff, &full_path, wide);
        if file_part_ptr != 0
            && let Some(pos) = full_path.rfind('\\')
        {
            let offset = (pos + 1) as u64 * 2;
            emu.maps.write_qword(file_part_ptr, buff + offset);
        }
        emu.regs_mut().rax = (needed - 1) as u64;
    } else {
        let needed = full_path.len() as u32 + 1;
        if size == 0 || buff == 0 {
            emu.regs_mut().rax = needed as u64;
            return;
        }
        if size < needed {
            emu.regs_mut().rax = needed as u64;
            return;
        }
        strings::write_string(emu, buff, &full_path, wide);
        if file_part_ptr != 0
            && let Some(pos) = full_path.rfind('\\')
        {
            let offset = pos as u64 + 1;
            emu.maps.write_qword(file_part_ptr, buff + offset);
        }
        emu.regs_mut().rax = (needed - 1) as u64;
    }
}
