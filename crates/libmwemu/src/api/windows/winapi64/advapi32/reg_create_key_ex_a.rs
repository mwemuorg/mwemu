use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _class_ptr = emu.regs().r9;
    let _options = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _sam = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _security = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let result_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let disposition_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let subkey = emu.maps.read_string(subkey_ptr);
    let new_handle = helper::handler_create(&format!("registry://{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, new_handle);
    }
    if disposition_ptr != 0 {
        emu.maps.write_dword(disposition_ptr, 2);
    }
    log_red!(
        emu,
        "advapi32!RegCreateKeyExA hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        new_handle
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
