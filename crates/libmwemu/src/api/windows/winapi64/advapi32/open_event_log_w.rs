use crate::emu;
use crate::winapi::helper;

pub fn OpenEventLogW(emu: &mut emu::Emu) {
    let _server_ptr = emu.regs().rcx;
    let source_ptr = emu.regs().rdx;
    let source = if source_ptr != 0 && source_ptr > 0xffff {
        emu.maps.read_wide_string(source_ptr)
    } else {
        String::from("Application")
    };
    let hndl = helper::handler_create(&format!("eventlog://{}", source));
    log_red!(
        emu,
        "advapi32!OpenEventLogW source:'{}' => 0x{:x}",
        source,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
