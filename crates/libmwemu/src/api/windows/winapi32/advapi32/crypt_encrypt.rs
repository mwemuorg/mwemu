use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0);
    let hhash = WinApiAbi::arg(emu, 1);
    let bfinal = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_len_ptr = WinApiAbi::arg(emu, 5);
    let buff_len = WinApiAbi::arg(emu, 6) as u32;

    if data_ptr != 0 && data_len_ptr != 0 {
        let data_len = emu.maps.read_dword(data_len_ptr).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(data_ptr, len).to_vec();
            crypto_state::encrypt_data(hkey, &mut data);
            emu.maps.write_bytes(data_ptr, &data);
        }
    }

    log_red!(emu, "advapi32!CryptEncrypt hKey=0x{:x}", hkey);

    for _ in 0..7 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
