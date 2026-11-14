use crate::emu;

pub fn SetClipboardData(emu: &mut emu::Emu) {
    let format = emu.regs().rcx;
    let hdata = emu.regs().rdx;
    log_red!(
        emu,
        "user32!SetClipboardData format:{} data:0x{:x}",
        format,
        hdata
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = hdata;
}
