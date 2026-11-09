use crate::emu;

pub fn InternetSetFilePointer(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let distance = emu.regs().rdx as i32;
    let method = emu.regs().r8 as u32;
    log_red!(
        emu,
        "wininet!InternetSetFilePointer handle:0x{:x} dist:{} method:{}",
        handle,
        distance,
        method
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
