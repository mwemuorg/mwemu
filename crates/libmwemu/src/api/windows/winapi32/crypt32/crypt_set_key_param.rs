use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSetKeyParam(emu: &mut emu::Emu) {
    let h_key = WinApiAbi::arg(emu, 0);
    let param = WinApiAbi::arg(emu, 1) as u32;
    let pb_data = WinApiAbi::arg(emu, 2);
    let _flags = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "crypt32!CryptSetKeyParam key:0x{:x} param:{} data:0x{:x}",
        h_key,
        param,
        pb_data
    );
    if param == crypto_state::KP_ALGID
        && pb_data != 0
        && let Some(key_len_data) = crypto_state::get_key_param(h_key, crypto_state::KP_KEYLEN)
    {
        let key_len = u32::from_le_bytes([
            key_len_data[0],
            key_len_data[1],
            key_len_data[2],
            key_len_data[3],
        ]) / 8;
        let data = emu.maps.read_bytes(pb_data, key_len as usize).to_vec();
        crypto_state::set_key_data(h_key, &data);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
