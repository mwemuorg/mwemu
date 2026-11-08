use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn BCryptGenRandom(emu: &mut emu::Emu) {
    let buf = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64;
    log_red!(emu, "bcrypt!BCryptGenRandom buf: 0x{:x} len: {}", buf, len);
    for i in 0..len {
        emu.maps
            .write_byte(buf + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 4);
}
