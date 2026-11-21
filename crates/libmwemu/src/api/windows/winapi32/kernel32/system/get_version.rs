use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::system::*;

pub fn GetVersion(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.set_return(emu, VERSION);
    log_red!(
        emu,
        "kernel32!GetVersion   =0x{:x}",
        emu.regs().get_eax() as u32
    );
}
