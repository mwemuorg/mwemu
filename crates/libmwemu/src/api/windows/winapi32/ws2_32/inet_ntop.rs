use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn inet_ntop(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!inet_ntop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    stdcall_ret(emu, 4);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InetNtopW(emu: &mut emu::Emu) {
    let family = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let str_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let str_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!InetNtopW family:{} addr:0x{:x} buf:0x{:x}",
        family,
        addr_ptr,
        str_buf
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
