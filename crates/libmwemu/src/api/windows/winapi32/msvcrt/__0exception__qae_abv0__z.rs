use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn __0exception__QAE_ABV0__Z(emu: &mut emu::Emu) {
    // exception::exception(const exception&) - thiscall copy ctor
    let this = emu.regs().get_ecx();
    let other = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if other != 0 && this != 0 {
        let vtable = emu.maps.read_dword(other).unwrap_or(0);
        let msg_ptr = emu.maps.read_dword(other + 4).unwrap_or(0);
        emu.maps.write_dword(this, vtable);
        emu.maps.write_dword(this + 4, msg_ptr);
    }
    log_red!(
        emu,
        "msvcrt!exception::exception(copy) this:0x{:x} other:0x{:x}",
        this,
        other
    );
    emu.regs_mut().rax = this;
    stdcall_ret(emu, 1);
}
