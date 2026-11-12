use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptUnprotectData(emu: &mut emu::Emu) {
    let p_data_in = emu.regs().rcx;
    let pp_sz_data_descr = emu.regs().rdx;
    let _p_optional_entropy = emu.regs().r8;
    let _pv_reserved = emu.regs().r9;
    let _p_prompt_struct = WinApiAbi::arg(emu, 4);
    let _dw_flags = WinApiAbi::arg(emu, 5);
    let p_data_out = WinApiAbi::arg(emu, 6);

    let mut in_len = 0u32;
    let mut in_ptr = 0u64;
    if p_data_in != 0 {
        in_len = emu.maps.read_dword(p_data_in).unwrap_or(0);
        in_ptr = emu.maps.read_qword(p_data_in + 8).unwrap_or(0);
    }

    log_red!(
        emu,
        "crypt32!CryptUnprotectData in_len:{} in_ptr:0x{:x} out:0x{:x}",
        in_len,
        in_ptr,
        p_data_out
    );

    if in_ptr != 0 && in_len > 0 && p_data_out != 0 {
        let len = in_len.min(0x100_0000) as usize;
        let data = emu.maps.read_bytes(in_ptr, len).to_vec();
        let unprotected = crypto_state::dpapi_unprotect(&data);
        if let Some(alloc) = emu.maps.alloc(unprotected.len() as u64) {
            emu.maps.write_bytes(alloc, &unprotected);
            emu.maps.write_dword(p_data_out, unprotected.len() as u32);
            emu.maps.write_qword(p_data_out + 8, alloc);
        }
    }
    if pp_sz_data_descr != 0 {
        emu.maps.write_qword(pp_sz_data_descr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
