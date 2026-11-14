use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn VariantClear(emu: &mut emu::Emu) {
    let variant_ptr = arg(emu, 0) as u64;

    log_red!(emu, "oleaut32!VariantClear ptr:0x{:x}", variant_ptr);

    if variant_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(variant_ptr + i * 4, 0);
        }
    }

    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
