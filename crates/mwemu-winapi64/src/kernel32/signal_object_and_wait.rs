use crate::emu;
use crate::winapi::helper;

pub fn SignalObjectAndWait(emu: &mut emu::Emu) {
    let object_to_signal = emu.regs().rcx;
    let object_to_wait = emu.regs().rdx;
    let millis = emu.regs().r8;
    let alertable = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!SignalObjectAndWait signal:0x{:x} wait:0x{:x} ms:{} alert:{}",
        object_to_signal,
        object_to_wait,
        millis,
        alertable
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
}
