use crate::emu;
use crate::winapi::helper;

pub fn EtwEventRegister(emu: &mut emu::Emu) {
    let provider_guid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let callback = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let callback_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let reg_handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
