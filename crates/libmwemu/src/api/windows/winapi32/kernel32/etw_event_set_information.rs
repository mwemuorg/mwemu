use crate::emu;

pub fn EtwEventSetInformation(emu: &mut emu::Emu) {
    let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let info_class = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let info_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let info_data = emu.maps.read_string(info_ptr);
    log_red!(
        emu,
        "kernel32!EventSetInformation handle:0x{:x} class:{} data:'{}' len:{}",
        reg_handle,
        info_class,
        info_data,
        info_len
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
