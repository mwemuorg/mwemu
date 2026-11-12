use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegRenameKey(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "advapi32!RegRenameKey h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    stdcall_ret(emu, 3);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
