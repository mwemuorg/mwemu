use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LocateXStateFeature(emu: &mut emu::Emu) {
    let context = arg(emu, 0) as u64;
    let feature_id = arg(emu, 1) as u64;
    let p_length = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!LocateXStateFeature ctx:0x{:x} feature:{} pLen:0x{:x}",
        context,
        feature_id,
        p_length
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
