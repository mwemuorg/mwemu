use crate::emu;
use crate::winapi::winapi64::msvcrt::release;

pub fn __3_YAXPAX_Z(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    release(emu, ptr);
}
