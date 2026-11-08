use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetThreadpoolWait(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!SetThreadpoolWait param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
