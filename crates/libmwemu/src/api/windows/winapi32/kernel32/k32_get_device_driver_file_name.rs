use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn K32GetDeviceDriverFileNameA(emu: &mut emu::Emu) {
    k32_get_device_driver_file_name(emu, "K32GetDeviceDriverFileNameA", false);
}

fn k32_get_device_driver_file_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}

pub fn K32GetDeviceDriverFileNameW(emu: &mut emu::Emu) {
    k32_get_device_driver_file_name(emu, "K32GetDeviceDriverFileNameW", true);
}
