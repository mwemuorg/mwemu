use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetFileVersionInfoA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lptstr_filename = api_try!(emu, call.arg(emu, 0));
    let dw_handle = api_try!(emu, call.arg(emu, 1)) as usize;
    let dw_len = api_try!(emu, call.arg(emu, 2)) as usize;
    let lp_data = api_try!(emu, call.arg(emu, 3)) as usize;

    let filename = if lptstr_filename > 0 {
        emu.maps.read_string(lptstr_filename)
    } else {
        "unknown".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoA filename: {} dw_handle: 0x{:x} dw_len: 0x{:x} lp_data: 0x{:x}",
        emu.pos,
        filename,
        dw_handle,
        dw_len,
        lp_data
    );

    if filename == "comctl32.dll" {
        use crate::windows::structures::{VS_FIXEDFILEINFO, VS_VERSIONINFO};

        let mut version_info = VS_VERSIONINFO::new();

        // Set comctl32.dll specific values based on the actual file
        version_info.value = VS_FIXEDFILEINFO {
            dw_signature: 0xFEEF04BD,
            dw_struc_version: 0x00010000,
            dw_file_version_ms: 0x0006000A,    // 6.10
            dw_file_version_ls: 0x585D11BD,    // 22621.4541
            dw_product_version_ms: 0x000A0000, // 10.0
            dw_product_version_ls: 0x585D11BD, // 22621.4541
            dw_file_flags_mask: 0x0000003F,
            dw_file_flags: 0x00000000,
            dw_file_os: 0x00040004,   // VOS_NT_WINDOWS32
            dw_file_type: 0x00000002, // VFT_DLL
            dw_file_subtype: 0x00000000,
            dw_file_date_ms: 0x00000000,
            dw_file_date_ls: 0x00000000,
        };

        version_info.write(emu, lp_data as u64);

        call.set_return(emu, 1); // Success
    } else {
        log::warn!("GetFileVersionInfoA: unsupported file {}", filename);
        call.set_return(emu, 0);
    }
}

pub fn GetFileVersionInfoW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernelbase!GetFileVersionInfoW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
