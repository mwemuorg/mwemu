use crate::api::windows::common::crypto_state;
use crate::emu;

pub(super) fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hhash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let bfinal = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let buff_len = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

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
