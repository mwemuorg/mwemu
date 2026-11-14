use crate::emu;

pub fn AllocateLocallyUniqueId(emu: &mut emu::Emu) {
    let luid_ptr = emu.regs().rcx;
    if luid_ptr != 0 {
        static LUID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1000);
        let low = LUID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        emu.maps.write_dword(luid_ptr, low);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(emu, "advapi32!AllocateLocallyUniqueId ptr:0x{:x}", luid_ptr);
    emu.regs_mut().rax = 1;
}
