use crate::emu;
use crate::winapi::helper;

pub fn EventWriteEx(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    log_red!(emu, "advapi32!EventWriteEx 0x{:x}", arg1);
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
