use super::UOI_FLAGS;
use super::UOI_NAME;
use super::UOI_TYPE;
use super::WSF_VISIBLE;
use super::wide_z;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetUserObjectInformationA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "user32!GetUserObjectInformationA '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub(super) fn GetUserObjectInformationW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hobj = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg(emu, 1)) & 0xffff_ffff;
    let info = api_try!(emu, call.arg(emu, 2));
    let length = api_try!(emu, call.arg(emu, 3)) & 0xffff_ffff;
    let needed_ptr = api_try!(emu, call.arg(emu, 4));

    log_red!(
        emu,
        "user32!GetUserObjectInformationW hObj: 0x{:x} nIndex: {} pvInfo: 0x{:x} nLength: {}",
        hobj,
        index,
        info,
        length
    );

    let data = match index {
        // USEROBJECTFLAGS { fInherit, fReserved, dwFlags }: an interactive, visible station
        UOI_FLAGS => [0u32, 0, WSF_VISIBLE]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>(),
        UOI_NAME => {
            let name = helper::handler_get_uri(hobj);
            let name = if name.is_empty() {
                "WinSta0".to_string()
            } else {
                name
            };
            wide_z(&name)
        }
        UOI_TYPE => wide_z("WindowStation"),
        _ => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            call.set_return(emu, 0);
            return;
        }
    };

    if needed_ptr != 0 {
        emu.maps.write_dword(needed_ptr, data.len() as u32);
    }
    if info == 0 || length < data.len() as u64 {
        emu.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        call.set_return(emu, 0);
        return;
    }

    emu.maps.write_bytes(info, &data);
    call.set_return(emu, 1);
}
