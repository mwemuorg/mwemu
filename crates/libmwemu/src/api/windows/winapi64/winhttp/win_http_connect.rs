use crate::emu;
use crate::winapi::helper;

pub fn WinHttpConnect(emu: &mut emu::Emu) {
    let h_session = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8 as u16;

    let server = if server_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(server_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        h_session,
        server,
        port
    );

    if !helper::handler_exist(h_session) {
        log::trace!("\tinvalid session handle");
    }

    let uri = format!("winhttp://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
