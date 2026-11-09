use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn WsaStartup(emu: &mut emu::Emu) {
    let version = arg(emu, 0);
    let wsadata_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "ws2_32!WsaStartup version:0x{:x} wsadata:0x{:x}",
        version,
        wsadata_ptr
    );
    if wsadata_ptr != 0 {
        emu.maps.write_word(wsadata_ptr, version as u16);
        emu.maps.write_word(wsadata_ptr + 2, version as u16);
        emu.maps.write_string(wsadata_ptr + 4, "WinSock 2.0");
        emu.maps.write_string(wsadata_ptr + 4 + 257, "Running");
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129, 512);
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129 + 2, 0);
    }
    stdcall_ret(emu, 2);
    emu.regs_mut().rax = 0;
}
