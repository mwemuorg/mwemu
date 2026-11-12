use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn QueryFullProcessImageNameA(emu: &mut emu::Emu) {
    query_full_process_image_name(emu, "QueryFullProcessImageNameA", false);
}

fn query_full_process_image_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}

pub fn QueryFullProcessImageNameW(emu: &mut emu::Emu) {
    query_full_process_image_name(emu, "QueryFullProcessImageNameW", true);
}
