use crate::emu;

pub fn GetJapaneseEraItems(emu: &mut emu::Emu) {
    let items_ptr = emu.regs().rcx;
    let count_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetJapaneseEraItems items:0x{:x} count:0x{:x}",
        items_ptr,
        count_ptr
    );
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
