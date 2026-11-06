
use crate::emu;
use crate::winapi::helper;
pub fn LZCopy(emu: &mut emu::Emu) {
    let hf_source = emu.regs().rcx;
    let hf_dest = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!LZCopy src:0x{:x} dest:0x{:x}",
        hf_source,
        hf_dest
    );
    let handle = helper::handler_create("LzCopy");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
