use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncGetHostByAddr(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let msg = emu.regs().rdx as u32;
    let addr_ptr = emu.regs().r8;
    let addr_len = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetHostByAddr hwnd:0x{:x} msg:{}",
        hwnd,
        msg
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
}
