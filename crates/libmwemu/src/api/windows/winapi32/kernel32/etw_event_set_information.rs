use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EtwEventSetInformation(emu: &mut emu::Emu) {
    let reg_handle = WinApiAbi::arg(emu, 0);
    let info_class = WinApiAbi::arg(emu, 1) as u32;
    let info_ptr = WinApiAbi::arg(emu, 2);
    let info_len = WinApiAbi::arg(emu, 3) as u32;
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
