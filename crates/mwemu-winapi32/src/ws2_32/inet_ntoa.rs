use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn inet_ntoa(emu: &mut emu::Emu) {
    let addr_val = arg(emu, 0);
    let a = addr_val & 0xFF;
    let b = (addr_val >> 8) & 0xFF;
    let c = (addr_val >> 16) & 0xFF;
    let d = (addr_val >> 24) & 0xFF;
    let ip_str = format!("{}.{}.{}.{}", a, b, c, d);
    log_red!(emu, "ws2_32!inet_ntoa -> {}", ip_str);
    let alloc_sz = 32u64;
    let buf = match emu.maps.alloc(alloc_sz) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
            return;
        }
    };
    emu.maps.create_map(
        "inet_ntoa_buf",
        buf,
        alloc_sz,
        crate::maps::mem64::Permission::READ_WRITE,
    );
    emu.maps.write_string(buf, &ip_str);
    emu.regs_mut().rax = buf;
    stdcall_ret(emu, 1);
}
