use crate::emu;

pub fn EraseTape(emu: &mut emu::Emu) {
    let h_device = emu.regs().rcx;
    let erase_type = emu.regs().rdx;
    let immediate = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!EraseTape hDevice:0x{:x} type:{} immediate:{}",
        h_device,
        erase_type,
        immediate
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
