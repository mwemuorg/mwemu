use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn __stdio_common_vsscanf(emu: &mut emu::Emu) {
    let _options = WinApiAbi::arg(emu, 0) as u32;
    let _options_hi = WinApiAbi::arg(emu, 1) as u32;
    let buf = WinApiAbi::arg(emu, 2);
    let _buf_size = WinApiAbi::arg(emu, 3) as u32;
    let fmt_ptr = WinApiAbi::arg(emu, 4);
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
        s,
        fmt
    );

    emu.regs_mut().rax = 1; // number of fields read
}
