use crate::emu;

pub fn IsProcessInJob(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let job = emu.regs().rdx;
    let result_ptr = emu.regs().r8;
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!IsProcessInJob proc:0x{:x} job:0x{:x} => false",
        process,
        job
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
