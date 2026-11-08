use crate::emu;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let h_exp_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_blob_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let pb_data = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pdw_data_len = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptExportKey key:0x{:x} exp:0x{:x} type:{} flags:0x{:x}",
        h_key,
        h_exp_key,
        dw_blob_type,
        dw_flags
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}
