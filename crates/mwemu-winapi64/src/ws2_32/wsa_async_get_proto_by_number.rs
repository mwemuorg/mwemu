use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncGetProtoByNumber(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let msg = emu.regs().rdx as u32;
    let number = emu.regs().r8 as u32;
    let buf_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetProtoByNumber hwnd:0x{:x} number:{}",
        hwnd,
        number
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
}
