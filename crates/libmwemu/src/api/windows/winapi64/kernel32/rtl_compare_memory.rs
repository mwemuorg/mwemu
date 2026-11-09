use crate::emu;
pub fn RtlCompareMemory(emu: &mut emu::Emu) {
    let source1 = emu.regs().rcx;
    let source2 = emu.regs().rdx;
    let length = emu.regs().r8 as usize;
    log_red!(
        emu,
        "kernel32!RtlCompareMemory s1:0x{:x} s2:0x{:x} len:{}",
        source1,
        source2,
        length
    );
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    let mut matching = 0usize;
    let len = length.min(0x100000);
    for i in 0..len {
        let b1 = emu.maps.read_byte(source1 + i as u64).unwrap_or(0);
        let b2 = emu.maps.read_byte(source2 + i as u64).unwrap_or(0);
        if b1 != b2 {
            break;
        }
        matching += 1;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = matching as u64;
}
