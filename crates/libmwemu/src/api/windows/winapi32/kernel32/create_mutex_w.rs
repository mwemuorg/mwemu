use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexW(emu: &mut emu::Emu) {
    let attr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateMutexW cannot read attr param");
    let owner = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateMutexW cannot read owner param");
    let name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateMutexW cannot read name param") as u64;
    let name = emu.maps.read_wide_string(name_ptr);

    log_red!(emu, "kernel32!CreateMutexW '{}'", name);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    let uri = format!("mutex://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
