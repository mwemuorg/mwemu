use crate::emu;
use crate::winapi::helper;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _sam = emu.regs().r9;
    let result_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let subkey = emu.maps.read_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!RegOpenKeyExA hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
