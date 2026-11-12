use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn QueryActCtxSettingsW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!QueryActCtxSettingsW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 7);
    emu.regs_mut().rax = 1;
}
