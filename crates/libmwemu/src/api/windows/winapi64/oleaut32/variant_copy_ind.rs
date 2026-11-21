use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VariantCopyInd(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pvarg_dest = api_try!(emu, call.arg(emu, 0));
    let pvarg_src = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "oleaut32!VariantCopyInd dest: 0x{:x} src: 0x{:x}",
        pvarg_dest,
        pvarg_src
    );
    if pvarg_dest != 0 && pvarg_src != 0 {
        for i in 0..16u64 {
            let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
            emu.maps.write_byte(pvarg_dest + i, b);
        }
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
