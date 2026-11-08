use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn WerSetFlagsWorker(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!WerSetFlagsWorker param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
