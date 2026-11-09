use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn LoadRegTypeLib(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    let _a1 = arg(emu, 1);
    let _a2 = arg(emu, 2);
    log_red!(emu, "oleaut32!LoadRegTypeLib a0:0x{:x} a1:0x{:x}", _a0, _a1);
    let hndl = helper::handler_create("oleaut32://LoadRegTypeLib");
    let out_ptr = arg(emu, 5) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, hndl as u32);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}
