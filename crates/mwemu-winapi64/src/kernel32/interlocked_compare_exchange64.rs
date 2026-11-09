use crate::emu;

pub fn InterlockedCompareExchange64(emu: &mut emu::Emu) {
    let dest_ptr = emu.regs().rcx;
    let exchange = emu.regs().rdx;
    let comparand = emu.regs().r8;

    let current = emu.maps.read_qword(dest_ptr).unwrap_or(0);

    if current == comparand {
        emu.maps.write_qword(dest_ptr, exchange);
    }

    log_red!(
        emu,
        "kernel32!InterlockedCompareExchange64 0x{:x} cur:{} cmp:{} xchg:{} => {}",
        dest_ptr,
        current,
        comparand,
        exchange,
        current
    );
    emu.regs_mut().rax = current;
}
