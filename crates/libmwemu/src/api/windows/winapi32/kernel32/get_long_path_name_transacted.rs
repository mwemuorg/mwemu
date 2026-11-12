use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetLongPathNameTransactedA(emu: &mut emu::Emu) {
    get_long_path_name_transacted(emu, "GetLongPathNameTransactedA", false);
}

fn get_long_path_name_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}

pub fn GetLongPathNameTransactedW(emu: &mut emu::Emu) {
    get_long_path_name_transacted(emu, "GetLongPathNameTransactedW", true);
}
