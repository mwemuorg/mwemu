use crate::emu;
use crate::winapi::helper;

pub fn WaitForMultipleObjects(emu: &mut emu::Emu) {
    let count = emu.regs().rcx;
    let _handles = emu.regs().rdx;
    let _wait_all = emu.regs().r8;
    let ms = emu.regs().r9;
    helper::advance_tick(emu, ms);
    log_red!(
        emu,
        "kernelbase!WaitForMultipleObjects count:{} ms:{} => WAIT_OBJECT_0",
        count,
        ms
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
}
