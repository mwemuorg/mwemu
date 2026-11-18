use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;

pub fn SafeArrayCreateVectorEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let vt = api_try!(emu, call.arg_u16(emu, 0));
    let lbound = api_try!(emu, call.arg_u32(emu, 1));
    let celements = api_try!(emu, call.arg_u32(emu, 2));
    let pvextra = api_try!(emu, call.arg(emu, 3));
    let elem_size: u64 = match vt {
        8 => 8,
        12 => 16,
        _ => 4,
    };
    let data_size = celements as u64 * elem_size;
    let alloc_size = 32 + data_size;
    let sa = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            call.set_return(emu, 0);
            return;
        }
    };
    emu.maps.create_map(
        &format!("safearray_{:x}", sa),
        sa,
        alloc_size,
        Permission::READ_WRITE,
    );
    emu.maps.write_word(sa, 1);
    emu.maps.write_word(sa + 2, 0);
    emu.maps.write_dword(sa + 4, elem_size as u32);
    emu.maps.write_dword(sa + 8, 0);
    emu.maps.write_qword(sa + 16, sa + 32);
    emu.maps.write_dword(sa + 24, celements);
    emu.maps.write_dword(sa + 28, lbound);
    log_red!(
        emu,
        "oleaut32!SafeArrayCreateVectorEx vt:{} count:{} =0x{:x}",
        vt,
        celements,
        sa
    );
    emu.set_last_error(0);
    call.set_return(emu, sa);
}
