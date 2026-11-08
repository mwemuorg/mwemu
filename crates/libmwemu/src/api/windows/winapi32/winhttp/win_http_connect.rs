use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WinHttpConnect(emu: &mut emu::Emu) {
    let session = arg(emu, 0);
    let server_ptr = arg(emu, 1) as u64;
    let port = arg(emu, 2) as u16;
    let server = if server_ptr != 0 {
        emu.maps.read_wide_string(server_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        session,
        server,
        port
    );
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://{}:{}", server, port));
    stdcall_ret(emu, 4);
}
