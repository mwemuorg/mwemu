use crate::emu;
use crate::winapi::helper;

pub fn WinHttpOpen(emu: &mut emu::Emu) {
    let agent_ptr = emu.regs().rcx;
    let _access_type = emu.regs().rdx as u32;
    let _proxy_ptr = emu.regs().r8;
    let _proxy_bypass_ptr = emu.regs().r9;

    let agent = if agent_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(agent_ptr)
    };

    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);

    let uri = format!("winhttp://session/{}", agent);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
