use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn NormalizeString(emu: &mut emu::Emu) {
    let norm_form = arg(emu, 0) as u64;
    let src_ptr = arg(emu, 1) as u64;
    let _src_len = arg(emu, 2) as u64 as i64;
    let dest_ptr = arg(emu, 3) as u64;
    let dest_len = arg(emu, 4) as u64 as i64;

    let src = emu.maps.read_wide_string(src_ptr);

    log_red!(
        emu,
        "kernel32!NormalizeString form:{} src:'{}' destLen:{}",
        norm_form,
        src,
        dest_len
    );

    let result_len = src.len() as i64;

    if dest_len == 0 || dest_ptr == 0 {
        emu.regs_mut().rax = (result_len + 1) as u64;
        return;
    }

    if dest_ptr != 0 && dest_len > 0 {
        emu.maps.write_wide_string(dest_ptr, &src);
    }
    emu.regs_mut().rax = (result_len + 1) as u64;
    stdcall_ret(emu, 5);
}
