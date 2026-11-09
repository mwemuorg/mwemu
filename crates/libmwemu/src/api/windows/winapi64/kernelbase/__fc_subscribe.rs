use crate::emu;
use crate::winapi::helper;

pub fn __FC_Subscribe(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!__FC_Subscribe {}:0x{:x} {}:0x{:x}",
        "param1",
        param1,
        "param2",
        param2
    );
    let handle = helper::handler_create("__FC_Subscribe");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // success
}
