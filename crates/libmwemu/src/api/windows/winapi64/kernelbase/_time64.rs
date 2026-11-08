use crate::emu;

pub fn _time64(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!_time64 {}:0x{:x} {}:0x{:x}",
        "param1",
        param1,
        "param2",
        param2
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // success
}
