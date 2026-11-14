use crate::emu;
use crate::winapi::common::strings;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let filepart_ptr = emu.regs().r9;
    let path = strings::read_text(emu, path_ptr, wide);
    let full = format!("C:\\mwemu\\{}", path);
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, &full, wide);
    }
    log_red!(emu, "kernelbase!{api} '{}' => '{}'", path, full);
    emu.set_last_error(0);
    emu.regs_mut().rax = full.len() as u64;
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}
