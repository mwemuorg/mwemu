use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _hexp_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let blob_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!CryptExportKey key: 0x{:x} type: {}",
        hkey,
        blob_type
    );
    let key_data = crypto_state::export_key(hkey);
    let blob_len = match &key_data {
        Some(d) => 12 + d.len() as u32,
        None => 12 + 16,
    };
    if data_ptr == 0 && data_len_ptr != 0 {
        emu.maps.write_dword(data_len_ptr, blob_len);
    } else if data_len_ptr != 0 {
        emu.maps.write_byte(data_ptr, blob_type as u8);
        emu.maps.write_byte(data_ptr + 1, 2);
        emu.maps.write_word(data_ptr + 2, 0);
        emu.maps.write_dword(data_ptr + 4, 0x00006801);
        emu.maps.write_dword(data_ptr + 8, 0);
        if let Some(ref kd) = key_data {
            for (i, b) in kd.iter().enumerate() {
                emu.maps.write_byte(data_ptr + 12 + i as u64, *b);
            }
        }
        emu.maps.write_dword(data_len_ptr, blob_len);
    }
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
