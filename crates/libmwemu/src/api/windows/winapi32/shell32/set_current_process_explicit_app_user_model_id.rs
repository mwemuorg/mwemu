use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetCurrentProcessExplicitAppUserModelID(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "shell32!SetCurrentProcessExplicitAppUserModelID arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
