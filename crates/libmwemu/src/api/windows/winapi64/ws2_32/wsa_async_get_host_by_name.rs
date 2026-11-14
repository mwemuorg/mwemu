use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncGetHostByName(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let msg = emu.regs().rdx as u32;
    let name_ptr = emu.regs().r8;
    let buf_ptr = emu.regs().r9;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetHostByName hwnd:0x{:x} name:'{}'",
        hwnd,
        name
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
}
