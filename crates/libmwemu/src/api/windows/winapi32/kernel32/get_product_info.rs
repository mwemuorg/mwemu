use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProductInfo(emu: &mut emu::Emu) {
    let os_major = arg(emu, 0);
    let os_minor = arg(emu, 1);
    let sp_major = arg(emu, 2);
    let sp_minor = arg(emu, 3);
    let product_type_ptr = arg(emu, 4) as u64;
    log_red!(
        emu,
        "kernel32!GetProductInfo {}.{} SP{}.{} ptr:0x{:x}",
        os_major,
        os_minor,
        sp_major,
        sp_minor,
        product_type_ptr
    );
    if product_type_ptr != 0 {
        emu.maps.write_dword(product_type_ptr, 0x30);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
