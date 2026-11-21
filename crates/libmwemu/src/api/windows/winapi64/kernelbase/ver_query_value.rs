use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn VerQueryValueA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_block = api_try!(emu, call.arg(emu, 0)) as usize;
    let lp_sub_block = api_try!(emu, call.arg(emu, 1));
    let lplp_buffer = api_try!(emu, call.arg(emu, 2)) as usize;
    let pu_len = api_try!(emu, call.arg(emu, 3)) as usize;

    let sub_block = if lp_sub_block > 0 {
        emu.maps.read_string(lp_sub_block)
    } else {
        "\\".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!VerQueryValueA p_block: 0x{:x} lp_sub_block: {} lplp_buffer: 0x{:x} pu_len: 0x{:x}",
        emu.pos,
        p_block,
        sub_block,
        lplp_buffer,
        pu_len
    );

    if sub_block == "\\" {
        // Root query returns pointer to VS_FIXEDFILEINFO
        // The VS_FIXEDFILEINFO starts at offset 0x28 in the version block
        let fixed_info_ptr = (p_block + 0x28) as u64;
        emu.maps.write_qword(lplp_buffer as u64, fixed_info_ptr);
        emu.maps.write_dword(pu_len as u64, 52); // Size of VS_FIXEDFILEINFO
        call.set_return(emu, 1);
    } else if sub_block.starts_with("\\StringFileInfo\\") {
        // String queries - allocate and return string data
        let string_data = match sub_block.as_str() {
            "\\StringFileInfo\\040904B0\\CompanyName" => "Microsoft Corporation\0",
            "\\StringFileInfo\\040904B0\\FileDescription" => "User Experience Controls Library\0",
            "\\StringFileInfo\\040904B0\\FileVersion" => "6.10 (WinBuild.160101.0800)\0",
            "\\StringFileInfo\\040904B0\\InternalName" => "comctl32\0",
            "\\StringFileInfo\\040904B0\\LegalCopyright" => {
                "© Microsoft Corporation. All rights reserved.\0"
            }
            "\\StringFileInfo\\040904B0\\OriginalFilename" => "comctl32.DLL\0",
            "\\StringFileInfo\\040904B0\\ProductName" => "Microsoft® Windows® Operating System\0",
            "\\StringFileInfo\\040904B0\\ProductVersion" => "10.0.22621.4541\0",
            _ => "\0",
        };

        let string_addr = match emu.maps.alloc(string_data.len() as u64) {
            Some(a) => a,
            None => {
                log::error!("kernelbase!VerQueryValueA: out of memory");
                call.set_return(emu, 0);
                return;
            }
        };
        emu.maps.write_string(string_addr, string_data);
        emu.maps.write_qword(lplp_buffer as u64, string_addr);
        emu.maps
            .write_dword(pu_len as u64, string_data.len() as u32);
        call.set_return(emu, 1);
    } else if sub_block == "\\VarFileInfo\\Translation" {
        // Translation array
        let trans_addr = match emu.maps.alloc(4) {
            Some(a) => a,
            None => {
                log::error!("kernelbase!VerQueryValueA: out of memory (translation)");
                call.set_return(emu, 0);
                return;
            }
        };
        emu.maps.write_dword(trans_addr, 0x04B00409); // Language and codepage
        emu.maps.write_qword(lplp_buffer as u64, trans_addr);
        emu.maps.write_dword(pu_len as u64, 4);
        call.set_return(emu, 1);
    } else {
        log::trace!("VerQueryValueA: Unknown sub_block: {}", sub_block);
        call.set_return(emu, 0); // Failure
    }
}

pub fn VerQueryValueW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!VerQueryValueW '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
