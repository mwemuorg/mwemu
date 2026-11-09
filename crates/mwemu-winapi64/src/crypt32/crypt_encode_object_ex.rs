use crate::emu;

pub fn CryptEncodeObjectEx(emu: &mut emu::Emu) {
    let encoding = emu.regs().rcx;
    let struct_type = emu.regs().rdx;
    let struct_info = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
        encoding,
        struct_type,
        flags
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
