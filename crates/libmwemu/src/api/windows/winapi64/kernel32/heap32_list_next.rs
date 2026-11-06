use crate::emu;
use crate::windows::constants;

pub fn Heap32ListNext(emu: &mut emu::Emu) {
    let _hSnapshot = emu.regs().rcx as u32;
    let _lphl = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!Heap32ListNext");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
