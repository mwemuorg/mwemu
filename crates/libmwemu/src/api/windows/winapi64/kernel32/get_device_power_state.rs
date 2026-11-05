use crate::emu;

pub fn GetDevicePowerState(emu: &mut emu::Emu) {
    let h_device = emu.regs().rcx;
    let pf_on = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetDevicePowerState h:0x{:x} pf_on:0x{:x}",
        h_device,
        pf_on
    );
    if pf_on != 0 {
        emu.maps.write_dword(pf_on, 1);
    }
    emu.regs_mut().rax = 1;
}
