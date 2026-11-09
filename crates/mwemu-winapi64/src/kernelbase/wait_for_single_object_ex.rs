use crate::emu;
use crate::winapi::helper;

pub fn WaitForSingleObjectEx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ms = emu.regs().rdx;
    let alertable = emu.regs().r8;
    helper::advance_tick(emu, ms);
    log_red!(
        emu,
        "kernelbase!WaitForSingleObjectEx handle:0x{:x} ms:{} alertable:{}",
        handle,
        ms,
        alertable
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
