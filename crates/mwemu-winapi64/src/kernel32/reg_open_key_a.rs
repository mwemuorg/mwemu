use crate::emu;
use crate::winapi::helper;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let result_ptr = emu.regs().r8;

    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };

    let hndl = helper::handler_create(&format!("registry://{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(emu, "kernel32!RegOpenKeyA '{}' => 0x{:x}", subkey, hndl);
    emu.regs_mut().rax = 0;
}
