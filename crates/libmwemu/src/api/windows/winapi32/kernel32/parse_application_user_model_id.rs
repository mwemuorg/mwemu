use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ParseApplicationUserModelId(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!ParseApplicationUserModelId param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );
    let src = arg(emu, 0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 15700;
    stdcall_ret(emu, 3);
}
