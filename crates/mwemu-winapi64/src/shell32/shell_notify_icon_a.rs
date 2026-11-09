use crate::emu;

pub fn Shell_NotifyIconA(emu: &mut emu::Emu) {
    let dw_message = emu.regs().rcx as u32;
    let lp_data = emu.regs().rdx;
    let tip = if lp_data != 0 {
        emu.maps.read_string(lp_data + 88)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!Shell_NotifyIconA msg:{} tip:'{}' data:0x{:x}",
        dw_message,
        tip,
        lp_data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
