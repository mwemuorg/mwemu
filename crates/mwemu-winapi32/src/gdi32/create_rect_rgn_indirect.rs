use crate::emu;
use crate::winapi::helper;

pub fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "gdi32!CreateRectRgnIndirect arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateRectRgnIndirect");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}
