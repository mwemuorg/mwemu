use crate::emu;
use crate::windows::constants::errors::*;

use crate::windows::structures::UnicodeString;

pub(super) fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if !emu.maps.is_mapped(dst_ptr) {
        log_red!(
            emu,
            "ntoskrnl!RtlInitUnicodeString worng destination pointer 0x{:x}",
            dst_ptr
        );
        for _ in 0..2 {
            emu.stack_pop32(false);
        }
        emu.regs_mut().rax = 0;
        return;
    }

    let mut s = "".to_string();
    let ustr;

    if src_ptr > 0 && emu.maps.is_mapped(src_ptr) {
        ustr = UnicodeString::load(src_ptr, &emu.maps);
        if emu.maps.is_mapped(ustr.buffer as u64) {
            s = emu.maps.read_wide_string(ustr.buffer as u64);
        } else {
            log_red!(
                emu,
                "ntoskrnl!RtlInitUnicodeString ustr.buffer is not ok: 0x{:x}",
                ustr.buffer
            );
        }
    } else {
        ustr = UnicodeString::new();
    }

    ustr.save(dst_ptr, &mut emu.maps);

    log_red!(
        emu,
        "ntoskrnl!RtlInitUnicodeString dst: 0x{:x} str:'{}' src: 0x{:x}",
        dst_ptr,
        s,
        src_ptr
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = STATUS_SUCCESS;
}
