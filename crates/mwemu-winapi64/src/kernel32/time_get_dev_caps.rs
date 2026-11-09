use crate::emu;

pub fn timeGetDevCaps(emu: &mut emu::Emu) {
    let p_time_caps = emu.regs().rcx;
    let cb_size = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!timeGetDevCaps pTimeCaps:0x{:x} cbSize:{}",
        p_time_caps,
        cb_size
    );

    if p_time_caps != 0 {
        emu.maps.write_dword(p_time_caps, 1);
        emu.maps.write_dword(p_time_caps + 4, 1000000);
    }
    emu.regs_mut().rax = 0;
}
