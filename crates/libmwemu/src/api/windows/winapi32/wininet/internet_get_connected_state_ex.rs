use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn InternetGetConnectedStateEx(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!InternetGetConnectedStateEx arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn InternetGetConnectedStateExA(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExA");
}

fn internet_get_connected_state_ex(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn InternetGetConnectedStateExW(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExW");
}
