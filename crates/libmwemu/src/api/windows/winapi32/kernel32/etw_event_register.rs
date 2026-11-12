use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn EtwEventRegister(emu: &mut emu::Emu) {
    let provider_guid = WinApiAbi::arg(emu, 0);
    let callback = WinApiAbi::arg(emu, 1) as u32;
    let callback_ctx = WinApiAbi::arg(emu, 2) as u32;
    let reg_handle_ptr = WinApiAbi::arg(emu, 3);
    let guid_str = emu.maps.read_string(provider_guid);
    let hndl = helper::handler_create(&guid_str);
    if reg_handle_ptr != 0 {
        emu.maps.write_dword(reg_handle_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "kernel32!EventRegister guid:'{}' cb:0x{:x} => handle:0x{:x}",
        guid_str,
        callback,
        hndl
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
