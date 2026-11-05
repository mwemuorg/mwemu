use crate::emu;

pub fn GetGeoInfoEx(emu: &mut emu::Emu) {
    let location_ptr = emu.regs().rcx;
    let geo_type = emu.regs().rdx as u32;
    let data_ptr = emu.regs().r8;
    let data_count = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!GetGeoInfoEx loc:0x{:x} type:{} data:0x{:x} count:{}",
        location_ptr,
        geo_type,
        data_ptr,
        data_count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
