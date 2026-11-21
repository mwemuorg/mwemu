use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn TlsGetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_u32(emu, 0));

    call.cleanup_args(emu, 1);

    if idx as usize >= emu.tls32().len() {
        call.set_return(emu, 0);
    } else {
        let tls_entry = emu.tls32()[idx as usize] as u64;
        call.set_return(emu, tls_entry);
    }

    log_red!(
        emu,
        "** {} kernel32!TlsGetValue idx: {} =0x{:x}",
        emu.pos,
        idx,
        emu.regs().get_eax() as u32
    );
}
