use crate::emu;
use crate::winapi::winapi32::msvcrt::release;

pub fn __3_YAXPAX_Z(emu: &mut emu::Emu) {
    // operator delete(void*)
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
    if ptr != 0 {
        if emu.cfg.memory_guard {
            emu.kernel_free(ptr, "operator_delete");
        } else {
            release(emu, ptr);
        }
    }
}
