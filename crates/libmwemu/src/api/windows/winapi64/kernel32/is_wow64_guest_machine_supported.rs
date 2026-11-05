use crate::emu;

pub fn IsWow64GuestMachineSupported(emu: &mut emu::Emu) {
    let machine = emu.regs().rcx;
    let p_supported = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!IsWow64GuestMachineSupported machine:0x{:x} pSupported:0x{:x}",
        machine,
        p_supported
    );
    if p_supported != 0 {
        emu.maps.write_dword(p_supported, 0);
    }
    emu.regs_mut().rax = 0;
}
