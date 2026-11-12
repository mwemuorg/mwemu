use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetProtoByNumber(emu: &mut emu::Emu) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let msg = WinApiAbi::arg(emu, 1) as u32;
    let number = WinApiAbi::arg(emu, 2) as u32;
    let buf_ptr = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetProtoByNumber hwnd:0x{:x} number:{}",
        hwnd,
        number
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 5);
}
