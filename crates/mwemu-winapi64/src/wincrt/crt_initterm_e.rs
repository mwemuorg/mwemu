use crate::emu;

pub(super) fn crt_initterm_e(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    let end = emu.regs().rdx;
    let mut count = 0u32;
    if start != 0 && end > start {
        let mut ptr = start;
        while ptr < end {
            let func = emu.maps.read_qword(ptr).unwrap_or(0);
            if func != 0 {
                count += 1;
            }
            ptr += 8;
        }
    }
    log_red!(
        emu,
        "wincrt!_initterm_e 0x{:x} - 0x{:x} ({} entries)",
        start,
        end,
        count
    );
    emu.regs_mut().rax = 0;
}
