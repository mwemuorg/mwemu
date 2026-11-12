use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _CxxThrowException(emu: &mut emu::Emu) {
    let exc_object = WinApiAbi::arg(emu, 0) as u32;
    let throw_info = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "msvcrt!_CxxThrowException obj:0x{:x} info:0x{:x}",
        exc_object,
        throw_info
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
