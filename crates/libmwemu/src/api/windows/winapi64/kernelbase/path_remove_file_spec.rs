use crate::emu;
use crate::winapi::common::strings;

pub fn PathRemoveFileSpecA(emu: &mut emu::Emu) {
    path_remove_file_spec(emu, "PathRemoveFileSpecA", false);
}

pub fn PathRemoveFileSpecW(emu: &mut emu::Emu) {
    path_remove_file_spec(emu, "PathRemoveFileSpecW", true);
}

fn path_remove_file_spec(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} path: '{}'", path);
    if wide {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.maps.write_word(path_ptr + (pos as u64) * 2, 0);
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.maps.write_byte(path_ptr + pos as u64, 0);
            emu.regs_mut().rax = 1; // TRUE, spec removed
        } else {
            emu.regs_mut().rax = 0; // FALSE
        }
    }
}
