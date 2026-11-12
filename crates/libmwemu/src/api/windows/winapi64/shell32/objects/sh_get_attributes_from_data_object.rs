use crate::emu;

pub fn SHGetAttributesFromDataObject(emu: &mut emu::Emu) {
    let pdtobj = emu.regs().rcx;
    let dw_attr_mask = emu.regs().rdx;
    let pdw_attr = emu.regs().r8;
    let pc_items = emu.regs().r9;
    log_red!(
        emu,
        "shell32!SHGetAttributesFromDataObject obj:0x{:x} mask:0x{:x}",
        pdtobj,
        dw_attr_mask
    );
    if pdw_attr != 0 {
        emu.maps.write_dword(pdw_attr, dw_attr_mask as u32);
    }
    if pc_items != 0 {
        emu.maps.write_dword(pc_items, 1);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
