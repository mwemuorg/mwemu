use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SignalObjectAndWait(emu: &mut emu::Emu) {
    let object_to_signal = arg(emu, 0) as u64;
    let object_to_wait = arg(emu, 1) as u64;
    let millis = arg(emu, 2) as u64;
    let alertable = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!SignalObjectAndWait signal:0x{:x} wait:0x{:x} ms:{} alert:{}",
        object_to_signal,
        object_to_wait,
        millis,
        alertable
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
    stdcall_ret(emu, 4);
}
