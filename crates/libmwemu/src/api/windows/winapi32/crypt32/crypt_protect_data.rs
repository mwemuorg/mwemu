use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptProtectData(emu: &mut emu::Emu) {
    let data_in = arg(emu, 0) as u64;
    let desc_ptr = arg(emu, 1) as u64;
    let _entropy = arg(emu, 2);
    let _reserved = arg(emu, 3);
    let _prompt = arg(emu, 4);
    let flags = arg(emu, 5);
    let data_out = arg(emu, 6) as u64;
    let desc = if desc_ptr != 0 {
        emu.maps.read_wide_string(desc_ptr)
    } else {
        String::new()
    };
    let mut in_len = 0u32;
    let mut in_ptr = 0u64;
    if data_in != 0 {
        in_len = emu.maps.read_dword(data_in).unwrap_or(0);
        in_ptr = emu.maps.read_dword(data_in + 4).unwrap_or(0) as u64;
    }
    log_red!(
        emu,
        "crypt32!CryptProtectData desc:'{}' in_len:{} in_ptr:0x{:x} flags:0x{:x}",
        desc,
        in_len,
        in_ptr,
        flags
    );
    if in_ptr != 0 && in_len > 0 && data_out != 0 {
        let len = in_len.min(0x100_0000) as usize;
        let data = emu.maps.read_bytes(in_ptr, len).to_vec();
        let protected = crypto_state::dpapi_protect(&data);
        if let Some(alloc) = emu.maps.alloc(protected.len() as u64) {
            emu.maps.write_bytes(alloc, &protected);
            emu.maps.write_dword(data_out, protected.len() as u32);
            emu.maps.write_dword(data_out + 4, alloc as u32);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
