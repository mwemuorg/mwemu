use crate::emu;

pub fn Wow64TransitionResolve(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Wow64TransitionResolve param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
