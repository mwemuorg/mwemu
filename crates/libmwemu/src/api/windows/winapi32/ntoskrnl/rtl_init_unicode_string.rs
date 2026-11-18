use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

use crate::windows::structures::UnicodeString;

pub(super) fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst_ptr = api_try!(emu, call.arg(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));

    if !emu.maps.is_mapped(dst_ptr) {
        log_red!(
            emu,
            "ntoskrnl!RtlInitUnicodeString worng destination pointer 0x{:x}",
            dst_ptr
        );
        call.cleanup_args(emu, 2);
        call.set_return(emu, 0);
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

    call.cleanup_args(emu, 2);

    call.set_return(emu, STATUS_SUCCESS);
}
