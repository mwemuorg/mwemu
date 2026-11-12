use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetHostByAddr(emu: &mut emu::Emu) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let msg = WinApiAbi::arg(emu, 1) as u32;
    let addr_ptr = WinApiAbi::arg(emu, 2);
    let addr_len = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetHostByAddr hwnd:0x{:x} msg:{}",
        hwnd,
        msg
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 7);
}
