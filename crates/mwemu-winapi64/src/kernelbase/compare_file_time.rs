use crate::emu;

pub fn CompareFileTime(emu: &mut emu::Emu) {
    let ft1_ptr = emu.regs().rcx;
    let ft2_ptr = emu.regs().rdx;
    let ft1_lo = emu.maps.read_dword(ft1_ptr).unwrap_or(0) as u64;
    let ft1_hi = emu.maps.read_dword(ft1_ptr + 4).unwrap_or(0) as u64;
    let ft2_lo = emu.maps.read_dword(ft2_ptr).unwrap_or(0) as u64;
    let ft2_hi = emu.maps.read_dword(ft2_ptr + 4).unwrap_or(0) as u64;
    let t1 = (ft1_hi << 32) | ft1_lo;
    let t2 = (ft2_hi << 32) | ft2_lo;
    let result: i32 = match t1.cmp(&t2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    log_red!(
        emu,
        "kernelbase!CompareFileTime 0x{:x} vs 0x{:x} => {}",
        t1,
        t2,
        result
    );
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u32 as u64;
}
