use crate::emu;
use crate::winapi::helper;

pub fn EtwEventRegister(emu: &mut emu::Emu) {
    let provider_guid = emu.regs().rcx;
    let callback = emu.regs().rdx;
    let _callback_ctx = emu.regs().r8;
    let reg_handle_ptr = emu.regs().r9;
    let guid_str = emu.maps.read_string(provider_guid);
    let hndl = helper::handler_create(&guid_str);
    if reg_handle_ptr != 0 {
        emu.maps.write_qword(reg_handle_ptr, hndl);
    }
    log_red!(
        emu,
        "kernel32!EventRegister guid:'{}' cb:0x{:x} => handle:0x{:x}",
        guid_str,
        callback,
        hndl
    );
    emu.regs_mut().rax = 0;
}
