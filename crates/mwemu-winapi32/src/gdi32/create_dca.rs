use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDCA(emu: &mut emu::Emu) {
    let driver = arg(emu, 0) as u64;
    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        emu.maps.read_string(device)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!CreateDCA device:'{}'", name);
    let hndl = helper::handler_create("gdi32://CreateDCA");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}
