use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysStringByteLen(emu: &mut emu::Emu) {
    let bstr = arg(emu, 0) as u64;

    let len = if bstr >= 8 {
        emu.maps.read_dword(bstr - 4).unwrap_or(0)
    } else {
        0
    };

    log_red!(emu, "oleaut32!SysStringByteLen 0x{:x} = {}", bstr, len);

    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 1);
}
