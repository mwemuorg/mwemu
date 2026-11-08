use crate::emu;
use crate::winapi::helper;

pub fn SysReAllocString(emu: &mut emu::Emu) {
    let bstr_ptr = emu.regs().rcx;
    let str_ptr = emu.regs().rdx;
    let s = emu.maps.read_wide_string(str_ptr);
    let hndl = helper::handler_create(&format!("bstr_{}", s));
    if bstr_ptr != 0 {
        emu.maps.write_qword(bstr_ptr, hndl);
    }
    log_red!(emu, "oleaut32!SysReAllocString '{}' => 0x{:x}", s, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
