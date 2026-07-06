use crate::emu;
use crate::windows::constants;

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    let hKey = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;
    let subkey_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;
    let reserved = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;
    let class_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;
    let options = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;
    let security_attr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!RegCreateKeyExW: error reading param") as u64;

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let mut class_name = "".to_string();
    if class_ptr > 0 {
        class_name = emu.maps.read_wide_string(class_ptr);
    }

    log_red!(emu, "kernel32!RegCreateKeyExW {} {}", subkey, class_name);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..9 {
        emu.stack_pop32(false);
    }
}
