use crate::emu;
use crate::winapi::helper;

pub fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let h_connect = emu.regs().rcx;
    let verb_ptr = emu.regs().rdx;
    let object_ptr = emu.regs().r8;
    let _version_ptr = emu.regs().r9;

    let verb = if verb_ptr == 0 {
        "GET".to_string()
    } else {
        emu.maps.read_wide_string(verb_ptr)
    };

    let object = if object_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(object_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        h_connect,
        verb,
        object
    );

    if !helper::handler_exist(h_connect) {
        log::trace!("\tinvalid connect handle");
    }

    let uri = format!("winhttp://request/{} {}", verb, object);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
