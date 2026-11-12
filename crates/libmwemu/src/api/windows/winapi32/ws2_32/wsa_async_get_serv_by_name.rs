use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAsyncGetServByName(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetServByName hwnd:0x{:x} name:'{}'",
        hwnd,
        name
    );
    let handle = helper::handler_create("wsa_async_task");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 6);
}
