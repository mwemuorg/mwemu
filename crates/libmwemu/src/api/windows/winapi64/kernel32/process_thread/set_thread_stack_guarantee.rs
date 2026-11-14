use crate::emu;

pub fn SetThreadStackGuarantee(emu: &mut emu::Emu) {
    let stack_size_ptr = emu.regs().rcx;

    let requested = if stack_size_ptr != 0 {
        emu.maps.read_dword(stack_size_ptr).unwrap_or(0)
    } else {
        0
    };

    log_red!(
        emu,
        "kernel32!SetThreadStackGuarantee ptr:0x{:x} requested:{}",
        stack_size_ptr,
        requested
    );

    // Write back the current guarantee (simulate a reasonable default)
    if stack_size_ptr != 0 {
        emu.maps.write_dword(stack_size_ptr, 0x1000); // 4KB default
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
