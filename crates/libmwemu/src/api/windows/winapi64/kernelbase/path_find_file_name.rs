use crate::emu;
use crate::winapi::common::strings;

pub fn PathFindFileNameA(emu: &mut emu::Emu) {
    path_find_file_name(emu, "PathFindFileNameA", false);
}

pub fn PathFindFileNameW(emu: &mut emu::Emu) {
    path_find_file_name(emu, "PathFindFileNameW", true);
}

fn path_find_file_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} path: '{}'", path);
    if wide {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.set_last_error(0);
            emu.regs_mut().rax = path_ptr + ((pos as u64) + 1) * 2;
        } else {
            emu.regs_mut().rax = path_ptr;
        }
    } else {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.set_last_error(0);
            emu.regs_mut().rax = path_ptr + (pos as u64) + 1;
        } else {
            emu.regs_mut().rax = path_ptr;
        }
    }
}
