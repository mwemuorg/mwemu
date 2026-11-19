use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetAttributesFromDataObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pdtobj = api_try!(emu, call.arg(emu, 0));
    let dw_attr_mask = api_try!(emu, call.arg(emu, 1));
    let pdw_attr = api_try!(emu, call.arg(emu, 2));
    let pc_items = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "shell32!SHGetAttributesFromDataObject obj:0x{:x} mask:0x{:x}",
        pdtobj,
        dw_attr_mask
    );
    if pdw_attr != 0 {
        emu.maps.write_dword(pdw_attr, dw_attr_mask as u32);
    }
    if pc_items != 0 {
        emu.maps.write_dword(pc_items, 1);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
