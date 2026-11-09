use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncGetServByPort(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let msg = emu.regs().rdx as u32;
    let port = emu.regs().r8 as u32;
    let proto_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetServByPort hwnd:0x{:x} port:{}",
        hwnd,
        port
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
}
