use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn IsCharAlphaNumericA(emu: &mut emu::Emu) {
    let c = arg(emu, 0) as u8 as char;
    log_red!(emu, "kernelbase!IsCharAlphaNumericA char: {}", c);
    emu.regs_mut().rax = c.is_ascii_alphanumeric() as u64;
    stdcall_ret(emu, 1);
}
