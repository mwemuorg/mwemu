use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    let hKey = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!RegOpenKeyA: error reading param") as u64;
    let subkey_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!RegOpenKeyA: error reading param") as u64;
    let result = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!RegOpenKeyA: error reading param") as u64;

    let subkey = emu.maps.read_string(subkey_ptr);
    emu.maps.write_dword(
        result,
        helper::handler_create(&format!("key://{}", subkey)) as u32,
    );

    log_red!(emu, "kernel32!RegOpenKeyA `{}`", subkey);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
