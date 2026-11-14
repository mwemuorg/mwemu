use crate::emu;

pub fn AllocateLocallyUniqueId(emu: &mut emu::Emu) {
    let luid_ptr = emu.regs().rcx;
    static LUID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0x10000);
    let val = LUID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if luid_ptr != 0 {
        emu.maps.write_dword(luid_ptr, val as u32);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(
        emu,
        "kernelbase!AllocateLocallyUniqueId ptr:0x{:x} => 0x{:x}",
        luid_ptr,
        val
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
