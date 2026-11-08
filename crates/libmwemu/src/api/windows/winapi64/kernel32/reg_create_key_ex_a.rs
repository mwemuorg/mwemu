use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _class_ptr = emu.regs().r9;
    let result_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };

    let hndl = helper::handler_create(&format!("registry://{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(emu, "kernel32!RegCreateKeyExA '{}' => 0x{:x}", subkey, hndl);
    emu.regs_mut().rax = 0;
}
