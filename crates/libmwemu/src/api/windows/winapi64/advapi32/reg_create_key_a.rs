use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let phk_result = emu.regs().r8;
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!RegCreateKeyA key:0x{:x} sub:'{}'",
        hkey,
        subkey
    );
    let new_key = helper::handler_create("reg_key");
    if phk_result != 0 {
        emu.maps.write_dword(phk_result, new_key as u32);
    }
    emu.regs_mut().rax = 0;
}
