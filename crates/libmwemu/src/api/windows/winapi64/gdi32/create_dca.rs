use crate::emu;
use crate::winapi::helper;

pub fn CreateDCA(emu: &mut emu::Emu) {
    let driver_ptr = emu.regs().rcx;
    let device_ptr = emu.regs().rdx;
    let driver_name = if driver_ptr != 0 {
        emu.maps.read_string(driver_ptr)
    } else {
        String::new()
    };
    let device_name = if device_ptr != 0 {
        emu.maps.read_string(device_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateDCA driver:'{}' device:'{}'",
        driver_name,
        device_name
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}
