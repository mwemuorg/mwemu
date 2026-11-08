use crate::emu;
use crate::winapi::helper;

pub fn StartServiceW(emu: &mut emu::Emu) {
    let hsvc = emu.regs().rcx;
    let num_args = emu.regs().rdx;
    let args_ptr = emu.regs().r8;
    log_red!(
        emu,
        "advapi32!StartServiceW svc:0x{:x} nargs:{}",
        hsvc,
        num_args
    );
    let handle = helper::handler_create("StartServiceW");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    for _ in 0..3 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
