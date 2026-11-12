use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn InternetConfirmZoneCrossing(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!InternetConfirmZoneCrossing arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub fn InternetConfirmZoneCrossingA(emu: &mut emu::Emu) {
    internet_confirm_zone_crossing(emu, "InternetConfirmZoneCrossingA");
}

fn internet_confirm_zone_crossing(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub fn InternetConfirmZoneCrossingW(emu: &mut emu::Emu) {
    internet_confirm_zone_crossing(emu, "InternetConfirmZoneCrossingW");
}
