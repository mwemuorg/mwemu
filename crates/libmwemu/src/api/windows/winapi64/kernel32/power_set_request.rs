use crate::emu;

pub fn PowerSetRequest(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let request_type = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!PowerSetRequest handle:0x{:x} request_type:0x{:x}",
        handle,
        request_type
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
