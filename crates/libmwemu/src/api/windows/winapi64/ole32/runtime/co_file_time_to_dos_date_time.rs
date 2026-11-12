use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CoFileTimeToDosDateTime(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "ole32!CoFileTimeToDosDateTime arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let out_ptr = WinApiAbi::arg(emu, 2);
    if out_ptr != 0 {
        emu.maps.write_word(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
