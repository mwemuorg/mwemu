use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn Shell_GetImageLists(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "shell32!Shell_GetImageLists arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
    stdcall_ret(emu, 2);
}
