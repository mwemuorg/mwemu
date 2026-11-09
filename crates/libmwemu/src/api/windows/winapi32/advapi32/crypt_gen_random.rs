use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CryptGenRandom(emu: &mut emu::Emu) {
    let len = arg(emu, 1) as u64;
    let buf = arg(emu, 2) as u64;
    log_red!(emu, "advapi32!CryptGenRandom len={} buf=0x{:x}", len, buf);
    let tick = emu.tick as u8;
    for i in 0..len {
        emu.maps.write_byte(buf + i, tick.wrapping_add(i as u8));
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
