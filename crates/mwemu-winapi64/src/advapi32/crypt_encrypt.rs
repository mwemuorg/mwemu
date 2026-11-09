use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let _hhash = emu.regs().rdx;
    let _bfinal = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let data_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _buff_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

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

    emu.regs_mut().rax = 1;
}
