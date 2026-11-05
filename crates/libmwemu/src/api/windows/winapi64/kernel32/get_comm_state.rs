use crate::emu;

pub fn GetCommState(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let dcb_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCommState h:0x{:x} dcb:0x{:x}",
        h_file,
        dcb_ptr
    );
    if dcb_ptr != 0 {
        emu.maps.write_dword(dcb_ptr, 28);
        emu.maps.write_dword(dcb_ptr + 4, 9600);
    }
    emu.regs_mut().rax = 1;
}
