use crate::emu;

pub fn GetMachineTypeAttributes(emu: &mut emu::Emu) {
    let machine = emu.regs().rcx as u16;
    let attrs_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetMachineTypeAttributes machine:0x{:x} attrs:0x{:x}",
        machine,
        attrs_ptr
    );
    if attrs_ptr != 0 {
        emu.maps.write_dword(attrs_ptr, 0x10);
    }
    emu.regs_mut().rax = 0;
}
