use crate::emu;

pub fn timeGetSystemTime(emu: &mut emu::Emu) {
    let pmm_time = emu.regs().rcx;
    let cb_size = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!timeGetSystemTime pmmTime:0x{:x} cbSize:{}",
        pmm_time,
        cb_size
    );

    if pmm_time != 0 {
        emu.maps.write_dword(pmm_time, 1);
        emu.maps.write_dword(pmm_time + 4, emu.tick as u32);
    }
    emu.regs_mut().rax = 0;
}
