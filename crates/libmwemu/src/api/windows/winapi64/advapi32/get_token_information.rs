use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_handle = api_try!(emu, call.arg(emu, 0));
    let info_class = api_try!(emu, call.arg(emu, 1));
    let info_buf = api_try!(emu, call.arg(emu, 2));
    let buf_len = api_try!(emu, call.arg(emu, 3));
    let ret_len_ptr = api_try!(emu, call.arg(emu, 4));

    let needed: u32 = match info_class {
        1 => 44, // TokenUser: TOKEN_USER (SID ptr + attrs)
        2 => 68, // TokenGroups
        3 => 44, // TokenPrivileges
        20 => 4, // TokenElevationType: DWORD
        25 => 4, // TokenIntegrityLevel
        _ => 4,
    };

    if ret_len_ptr != 0 {
        emu.maps.write_dword(ret_len_ptr, needed);
    }

    if info_buf != 0 && buf_len >= needed as u64 {
        match info_class {
            20 => {
                emu.maps.write_dword(info_buf, 2); // TokenElevationTypeFull
            }
            25 => {
                emu.maps.write_dword(info_buf, 0x3000); // SECURITY_MANDATORY_HIGH_RID
            }
            _ => {
                emu.maps.write_dword(info_buf, 1);
            }
        }
    }

    log_red!(
        emu,
        "advapi32!GetTokenInformation token:0x{:x} class:{} needed:{}",
        token_handle,
        info_class,
        needed
    );
    call.set_return(emu, 1);
}
