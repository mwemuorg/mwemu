use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetServByPort(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let port = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetServByPort hwnd:0x{:x} port:{}",
        hwnd,
        port
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 6);
}
