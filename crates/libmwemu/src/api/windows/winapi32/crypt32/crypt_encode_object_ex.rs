use crate::emu;

pub fn CryptEncodeObjectEx(emu: &mut emu::Emu) {
    let encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let struct_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let struct_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p_encode_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pv_encoded = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let pcb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
        encoding,
        struct_type,
        flags
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
