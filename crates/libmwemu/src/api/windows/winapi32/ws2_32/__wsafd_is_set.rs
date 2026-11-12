use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn __WSAFDIsSet(emu: &mut emu::Emu) {
    let fd = WinApiAbi::arg(emu, 0);
    let set_ptr = WinApiAbi::arg(emu, 1);
    log_red!(emu, "ws2_32!__WSAFDIsSet fd:0x{:x} set:0x{:x}", fd, set_ptr);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
