use crate::emu;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    let query_set_ptr = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let flags = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!WSASetServiceA op:{} flags:0x{:x}",
        operation,
        flags
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
