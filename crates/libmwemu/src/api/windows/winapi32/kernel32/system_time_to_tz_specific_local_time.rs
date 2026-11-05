use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let _tz_ptr = arg(emu, 0) as u64;
    let ut_ptr = arg(emu, 1) as u64;
    let lt_ptr = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!SystemTimeToTzSpecificLocalTime utc:0x{:x} local:0x{:x}",
        ut_ptr,
        lt_ptr
    );

    if ut_ptr != 0 && lt_ptr != 0 {
        // Copy 16-byte SYSTEMTIME from UTC to local (Bias=0)
        for off in (0u64..16).step_by(2) {
            let val = emu.maps.read_word(ut_ptr + off).unwrap_or(0);
            emu.maps.write_word(lt_ptr + off, val);
        }
    }

    emu.set_last_error(0);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
