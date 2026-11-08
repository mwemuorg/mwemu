use crate::emu;

pub fn SafeArrayAccessData(emu: &mut emu::Emu) {
    let sa = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    if data_ptr != 0 {
        emu.maps.write_qword(data_ptr, sa + 0x20);
    }
    log_red!(emu, "oleaut32!SafeArrayAccessData 0x{:x}", sa);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
