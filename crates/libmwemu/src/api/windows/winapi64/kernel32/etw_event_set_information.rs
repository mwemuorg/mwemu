use crate::emu;

pub fn EtwEventSetInformation(emu: &mut emu::Emu) {
    let reg_handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let info_ptr = emu.regs().r8;
    let info_len = emu.regs().r9;
    let info_data = if info_ptr != 0 {
        emu.maps.read_string(info_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!EventSetInformation handle:0x{:x} class:{} data:'{}' len:{}",
        reg_handle,
        info_class,
        info_data,
        info_len
    );
    emu.regs_mut().rax = 0;
}
