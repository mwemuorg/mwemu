use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _controlfp(emu: &mut emu::Emu) {
    let new_val = WinApiAbi::arg(emu, 0) as u32;
    let mask = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "msvcrt!_controlfp new:0x{:x} mask:0x{:x}",
        new_val,
        mask
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0009_001f; // default x87 control word
}
