use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let connect = arg(emu, 0);
    let verb_ptr = arg(emu, 1) as u64;
    let object_ptr = arg(emu, 2) as u64;
    let verb = if verb_ptr != 0 {
        emu.maps.read_wide_string(verb_ptr)
    } else {
        "GET".to_string()
    };
    let object = if object_ptr != 0 {
        emu.maps.read_wide_string(object_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        connect,
        verb,
        object
    );
    emu.regs_mut().rax = helper::handler_create(&format!("winhttp://request/{} {}", verb, object));
    stdcall_ret(emu, 7);
}
