use crate::emu;

pub fn Shell_NotifyIcon(emu: &mut emu::Emu) {
    let dw_message = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lp_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let tip = if lp_data != 0 {
        emu.maps.read_string(lp_data + 64)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!Shell_NotifyIcon msg:{} tip:'{}' data:0x{:x}",
        dw_message,
        tip,
        lp_data
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
