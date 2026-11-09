use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn RtlCompareMemory(emu: &mut emu::Emu) {
    let source1 = arg(emu, 0) as u64;
    let source2 = arg(emu, 1) as u64;
    let length = arg(emu, 2) as u64 as usize;
    log_red!(
        emu,
        "kernel32!RtlCompareMemory s1:0x{:x} s2:0x{:x} len:{}",
        source1,
        source2,
        length
    );
    let h = arg(emu, 0) as u64;
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
    stdcall_ret(emu, 3);
}
