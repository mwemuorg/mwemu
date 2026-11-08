use crate::emu;

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let wow64_ptr = emu.regs().rdx;
    if wow64_ptr != 0 {
        emu.maps.write_dword(wow64_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!IsWow64Process handle:0x{:x} => FALSE",
        process
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
