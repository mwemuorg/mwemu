use crate::emu;

const MEM_RELEASE: u64 = 0x8000;

pub fn VirtualFreeEx(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let lp_address = emu.regs().rdx;
    let dw_size = emu.regs().r8;
    let dw_free_type = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!VirtualFreeEx hProcess:0x{:x} addr:0x{:x} size:{} type:0x{:x}",
        h_process,
        lp_address,
        dw_size,
        dw_free_type
    );

    if (dw_free_type & MEM_RELEASE) != 0 && lp_address != 0 {
        // Try to free the memory map allocated by VirtualAllocEx
        emu.maps.free(&format!("alloc_{:x}", lp_address));
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
