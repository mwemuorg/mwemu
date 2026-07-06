use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn CreateFileW(emu: &mut emu::Emu) {
    let fname_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateFileW: error reading param") as u64;
    let access = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateFileW: error reading param");

    let fname = emu.maps.read_wide_string(fname_ptr);

    let mut perm: String = String::new();
    if access & constants::GENERIC_READ != 0 {
        perm.push('r');
    }
    if access & constants::GENERIC_WRITE != 0 {
        perm.push('w');
    }

    if perm.is_empty() {
        perm = "unknown permissions".to_string();
    }

    log_red!(emu, "kernel32!CreateFileW `{}` {}", fname, perm);

    for _ in 0..7 {
        emu.stack_pop32(false);
    }

    //if perm == "r" {
    //    emu.regs_mut().rax = constants::INVALID_HANDLE_VALUE_32;
    //} else {
    emu.regs_mut().rax = helper::handler_create(&format!("file://{}", fname)) as u64;
    //}
}
