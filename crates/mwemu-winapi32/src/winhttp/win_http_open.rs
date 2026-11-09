use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WinHttpOpen(emu: &mut emu::Emu) {
    let agent_ptr = arg(emu, 0) as u64;
    let agent = if agent_ptr != 0 {
        emu.maps.read_wide_string(agent_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://session/{}", agent));
    stdcall_ret(emu, 5);
}
