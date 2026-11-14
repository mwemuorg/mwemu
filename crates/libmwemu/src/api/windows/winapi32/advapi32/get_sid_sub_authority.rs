use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetSidSubAuthority(emu: &mut emu::Emu) {
    let sid_ptr = WinApiAbi::arg(emu, 0);
    let index = WinApiAbi::arg(emu, 1);
    // Returns pointer to the sub-authority DWORD at offset 8 + index*4
    let result = sid_ptr + 8 + index * 4;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthority sid:0x{:x} idx:{} => 0x{:x}",
        sid_ptr,
        index,
        result
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
