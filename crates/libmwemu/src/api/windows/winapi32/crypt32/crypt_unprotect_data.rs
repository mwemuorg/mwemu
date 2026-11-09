use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptUnprotectData(emu: &mut emu::Emu) {
    let data_in = arg(emu, 0) as u64;
    let pp_desc = arg(emu, 1) as u64;
    let _entropy = arg(emu, 2);
    let _reserved = arg(emu, 3);
    let _prompt = arg(emu, 4);
    let flags = arg(emu, 5);
    let data_out = arg(emu, 6) as u64;
    let mut in_len = 0u32;
    let mut in_ptr = 0u64;
    if data_in != 0 {
        in_len = emu.maps.read_dword(data_in).unwrap_or(0);
        in_ptr = emu.maps.read_dword(data_in + 4).unwrap_or(0) as u64;
    }
    log_red!(
        emu,
        "crypt32!CryptUnprotectData in_len:{} in_ptr:0x{:x} flags:0x{:x}",
        in_len,
        in_ptr,
        flags
    );
    if in_ptr != 0 && in_len > 0 && data_out != 0 {
        let len = in_len.min(0x100_0000) as usize;
        let data = emu.maps.read_bytes(in_ptr, len).to_vec();
        let unprotected = crypto_state::dpapi_unprotect(&data);
        if let Some(alloc) = emu.maps.alloc(unprotected.len() as u64) {
            emu.maps.write_bytes(alloc, &unprotected);
            emu.maps.write_dword(data_out, unprotected.len() as u32);
            emu.maps.write_dword(data_out + 4, alloc as u32);
        }
    }
    if pp_desc != 0 {
        emu.maps.write_dword(pp_desc, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
