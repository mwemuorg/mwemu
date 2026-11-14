use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn GetFinalPathNameByHandleA(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleA", false);
}

fn get_final_path_name_by_handle(emu: &mut emu::Emu, api: &str, wide: bool) {
    let handle = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let uri = helper::handler_get_uri(handle);
    let path = uri.strip_prefix("file://").unwrap_or("C:\\unknown");
    log_red!(emu, "kernelbase!{api} h:0x{:x} => '{}'", handle, path);

    if buf != 0 && buf_size > 0 {
        strings::write_string(emu, buf, path, wide);
    }
    emu.regs_mut().rax = path.len() as u64;
}

pub fn GetFinalPathNameByHandleW(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleW", true);
}
