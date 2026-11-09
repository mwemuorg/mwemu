use crate::emu;

pub fn __0exception__QAE_ABV0__Z(emu: &mut emu::Emu) {
    let this_ptr = emu.regs().rcx;
    let other = emu.regs().rdx;
    if other != 0 && this_ptr != 0 {
        let vtable = emu.maps.read_qword(other).unwrap_or(0);
        let msg_ptr = emu.maps.read_qword(other + 8).unwrap_or(0);
        emu.maps.write_qword(this_ptr, vtable);
        emu.maps.write_qword(this_ptr + 8, msg_ptr);
    }
    log_red!(
        emu,
        "msvcrt!exception::exception(copy) this:0x{:x} other:0x{:x}",
        this_ptr,
        other
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = this_ptr;
}
