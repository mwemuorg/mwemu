use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let result_ptr = emu.regs().r8;

    let subkey = emu.maps.read_string(subkey_ptr);

    log_red!(
        emu,
        "kernel32!RegOpenKeyA hKey:0x{:x} subKey:`{}`",
        h_key,
        subkey
    );

    emu.maps.write_dword(
        result_ptr,
        helper::handler_create(&format!("key://{}", subkey)) as u32,
    );
    // Note: write_dword writes 4 bytes. HKEY is pointer sized (8 bytes on 64 bit).
    // However, handles are often treated as 32-bit or fit in 32-bit.
    // If `helper::handler_create` returns u32, this is fine.
    // But `result_ptr` is `PHKEY` which is `HKEY*`. `HKEY` is `HANDLE` (void*).
    // So on 64-bit it should be 8 bytes?
    // Let's use write_qword for correctness on 64-bit unless we are sure handles are 32-bit constants.
    // In `winapi32` it was `write_dword`.
    // Let's try `write_qword` but cast the handler to u64.

    emu.maps.write_qword(
        result_ptr,
        helper::handler_create(&format!("key://{}", subkey)) as u64,
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
