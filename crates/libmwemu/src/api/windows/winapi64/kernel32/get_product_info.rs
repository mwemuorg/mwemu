use crate::emu;

pub fn GetProductInfo(emu: &mut emu::Emu) {
    let os_major = emu.regs().rcx as u32;
    let os_minor = emu.regs().rdx as u32;
    let sp_major = emu.regs().r8 as u32;
    let sp_minor = emu.regs().r9 as u32;
    let product_type_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
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
}
