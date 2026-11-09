use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysStringLen(emu: &mut emu::Emu) {
    let bstr = arg(emu, 0) as u64;

    let len = if bstr >= 8 {
        let byte_len = emu.maps.read_dword(bstr - 4).unwrap_or(0);
        byte_len / 2
    } else {
        0
    };

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} = {}", bstr, len);

    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 1);
}
