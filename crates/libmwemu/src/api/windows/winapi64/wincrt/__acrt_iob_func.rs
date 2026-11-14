use super::allocate_memory;
use crate::emu;

pub(super) fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    if iob_addr != 0 {
        emu.maps.memset(iob_addr, 0, 96);
        let fd = match index {
            0 => 0u32, // stdin
            1 => 1,    // stdout
            2 => 2,    // stderr
            _ => index as u32,
        };
        emu.maps.write_dword(iob_addr + 16, fd); // _file field offset
    }
    log_red!(
        emu,
        "wincrt!__acrt_iob_func index:{} => 0x{:x}",
        index,
        iob_addr
    );
    emu.regs_mut().rax = iob_addr;
}
