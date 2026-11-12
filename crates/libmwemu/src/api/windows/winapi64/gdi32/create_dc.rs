use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateDCA(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCA", false);
}

fn create_dc(emu: &mut emu::Emu, api: &str, wide: bool) {
    let driver_ptr = emu.regs().rcx;
    let device_ptr = emu.regs().rdx;
    let driver_name = if driver_ptr != 0 {
        strings::read_text(emu, driver_ptr, wide)
    } else {
        String::new()
    };
    let device_name = if device_ptr != 0 {
        strings::read_text(emu, device_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} driver:'{}' device:'{}'",
        driver_name,
        device_name
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCW", true);
}
