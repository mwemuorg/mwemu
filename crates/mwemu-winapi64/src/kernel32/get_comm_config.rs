use crate::emu;

pub fn GetCommConfig(emu: &mut emu::Emu) {
    let h_comm_dev = emu.regs().rcx;
    let config_ptr = emu.regs().rdx;
    let size_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetCommConfig h:0x{:x} config:0x{:x} size:0x{:x}",
        h_comm_dev,
        config_ptr,
        size_ptr
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 52);
    }
    emu.regs_mut().rax = 1;
}
