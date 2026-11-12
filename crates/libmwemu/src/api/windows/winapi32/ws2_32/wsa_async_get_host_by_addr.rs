use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetHostByAddr(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
