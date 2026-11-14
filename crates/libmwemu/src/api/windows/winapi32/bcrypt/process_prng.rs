use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn ProcessPrng(emu: &mut emu::Emu) {
    let pb_data = arg(emu, 0) as u64;
    let cb_data = arg(emu, 1) as u64;
    log_red!(
        emu,
        "bcrypt!ProcessPrng buf:0x{:x} len:{}",
        pb_data,
        cb_data
    );
    for i in 0..cb_data {
        emu.maps
            .write_byte(pb_data + i, (emu.pos.wrapping_add(i) & 0xff) as u8);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
