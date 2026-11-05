use crate::emu;

pub fn GetThreadInformation(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let info_class = emu.regs().rdx as u32;
    let info_ptr = emu.regs().r8;
    let info_size = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!GetThreadInformation h:0x{:x} class:{} ptr:0x{:x} size:{}",
        h_thread,
        info_class,
        info_ptr,
        info_size
    );
    if info_ptr != 0 && info_size > 0 {
        let clear_size = (info_size as usize).min(256);
        for i in 0..clear_size {
            emu.maps.write_byte(info_ptr + i as u64, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
