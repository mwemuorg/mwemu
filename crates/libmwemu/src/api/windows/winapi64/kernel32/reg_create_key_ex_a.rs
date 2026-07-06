use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let class_ptr = emu.regs().r9;

    // Stack params (shadow space is 32 bytes, so 4 * 8)
    // RSP points to shadow space start (after we popped ret addr in handler)
    // So arguments start at RSP + 32
    // Arg 5 (dwOptions) at RSP + 32
    // Arg 6 (samDesired) at RSP + 40
    // Arg 7 (lpSecurityAttributes) at RSP + 48
    // Arg 8 (phkResult) at RSP + 56
    // Arg 9 (lpdwDisposition) at RSP + 64

    let _options = emu.maps.read_qword(emu.regs().rsp + 32).unwrap_or(0);
    let _sam_desired = emu.maps.read_qword(emu.regs().rsp + 40).unwrap_or(0);
    let _security_attr = emu.maps.read_qword(emu.regs().rsp + 48).unwrap_or(0);
    let result_ptr = emu.maps.read_qword(emu.regs().rsp + 56).unwrap_or(0);
    let _disposition_ptr = emu.maps.read_qword(emu.regs().rsp + 64).unwrap_or(0);

    let subkey = emu.maps.read_string(subkey_ptr);
    let mut class_name = String::new();
    if class_ptr > 0 {
        class_name = emu.maps.read_string(class_ptr);
    }

    log_red!(
        emu,
        "kernel32!RegCreateKeyExA {} class:{}",
        subkey,
        class_name
    );

    // Write a dummy handle to phkResult
    if result_ptr != 0 {
        emu.maps.write_qword(
            result_ptr,
            helper::handler_create(&format!("key://{}", subkey)) as u64,
        );
    }

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
