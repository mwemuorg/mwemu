use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0);
    let _hexp_key = WinApiAbi::arg(emu, 1) as u32;
    let blob_type = WinApiAbi::arg(emu, 2) as u32;
    let _flags = WinApiAbi::arg(emu, 3) as u32;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_len_ptr = WinApiAbi::arg(emu, 5);
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
