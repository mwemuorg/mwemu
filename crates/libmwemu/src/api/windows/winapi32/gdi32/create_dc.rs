use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDCA(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCA", false);
}

pub fn CreateDCW(emu: &mut emu::Emu) {
    create_dc(emu, "CreateDCW", true);
}

fn create_dc(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
    } else {
        let driver = arg(emu, 0) as u64;
    }

    let device = arg(emu, 1) as u64;
    let name = if device != 0 {
        strings::read_text(emu, device, wide)
    } else {
        String::new()
    };
    log_red!(emu, "gdi32!{api} device:'{}'", name);
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}
