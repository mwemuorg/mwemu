use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InterlockedCompareExchange64(emu: &mut emu::Emu) {
    let dest_ptr = arg(emu, 0) as u64;
    let exchange = arg(emu, 1) as u64;
    let comparand = arg(emu, 2) as u64;

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
    stdcall_ret(emu, 3);
}
