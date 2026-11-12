use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetProtoByName(emu: &mut emu::Emu) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let msg = WinApiAbi::arg(emu, 1) as u32;
    let name_ptr = WinApiAbi::arg(emu, 2);
    let buf_ptr = WinApiAbi::arg(emu, 3);
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetProtoByName hwnd:0x{:x} name:'{}'",
        hwnd,
        name
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 5);
}
