use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn inet_ntop(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(emu, "ws2_32!inet_ntop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    stdcall_ret(emu, 4);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InetNtopW(emu: &mut emu::Emu) {
    let family = WinApiAbi::arg(emu, 0) as u32;
    let addr_ptr = WinApiAbi::arg(emu, 1);
    let str_buf = WinApiAbi::arg(emu, 2);
    let str_len = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!InetNtopW family:{} addr:0x{:x} buf:0x{:x}",
        family,
        addr_ptr,
        str_buf
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if family == 2 && addr_ptr != 0 && str_buf != 0 {
        let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
        let a = ip & 0xFF;
        let b = (ip >> 8) & 0xFF;
        let c = (ip >> 16) & 0xFF;
        let d = (ip >> 24) & 0xFF;
        let s = format!("{}.{}.{}.{}", a, b, c, d);
        if (s.len() as u32) < str_len {
            emu.maps.write_wide_string(str_buf, &s);
            emu.regs_mut().rax = str_buf;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
