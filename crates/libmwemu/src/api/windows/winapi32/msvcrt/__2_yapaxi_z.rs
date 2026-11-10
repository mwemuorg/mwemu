use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn __2_YAPAXI_Z(emu: &mut emu::Emu, addr: u32) {
    // operator new(unsigned int)
    let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.stack_pop32(false);
    let addr = if size > 0 {
        allocate_memory(emu, size).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "msvcrt!operator_new sz:{} => 0x{:x}", size, addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
