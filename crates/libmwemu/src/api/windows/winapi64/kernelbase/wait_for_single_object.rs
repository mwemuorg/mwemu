use crate::emu;
use crate::winapi::helper;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ms = emu.regs().rdx;
    helper::advance_tick(emu, ms);
    log_red!(
        emu,
        "kernelbase!WaitForSingleObject handle:0x{:x} ms:{} => WAIT_OBJECT_0",
        handle,
        ms
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
}
