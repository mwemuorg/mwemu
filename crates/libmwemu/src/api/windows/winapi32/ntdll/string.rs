use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::api::dispatch_api;
use crate::api::windows::common::ntdll;
use crate::emu;
use crate::winapi::policy::api_try;
use scan_fmt::scan_fmt_some;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "strcat" => strcat(emu),
        "sscanf" => sscanf(emu),
        "RtlInitAnsiString" => RtlInitAnsiString(emu),
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn stricmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str1ptr = api_try!(emu, call.arg(emu, 0));
    let str2ptr = api_try!(emu, call.arg(emu, 1));
    ntdll::stricmp(emu, str1ptr, str2ptr);

    call.cleanup_args(emu, 2);
}

fn strlen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));

    let len = ntdll::strlen(emu, s_ptr);
    call.cleanup_args(emu, 1);
    call.set_return(emu, len as u32 as u64);
}

fn strcat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst_ptr = api_try!(emu, call.arg(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));

    let dst_cont_ptr = ntdll::strcat(emu, dst_ptr, src_ptr);

    call.cleanup_args(emu, 2);
    call.set_return(emu, dst_cont_ptr);
}

fn sscanf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buffer_ptr = api_try!(emu, call.arg(emu, 0));
    let fmt_ptr = api_try!(emu, call.arg(emu, 1));
    let list = api_try!(emu, call.arg_u32(emu, 2));

    let buffer = emu.maps.read_string(buffer_ptr);
    let fmt = emu.maps.read_string(fmt_ptr);

    log_red!(emu, "ntdll!sscanf out_buff: `{}` fmt: `{}`", buffer, fmt);

    let rust_fmt = fmt
        .replace("%x", "{x}")
        .replace("%d", "{}")
        .replace("%s", "{}")
        .replace("%hu", "{u16}")
        .replace("%i", "{}")
        .replace("%o", "{o}")
        .replace("%f", "{}");

    let params = rust_fmt.matches("{").count();
    let b = buffer.as_str();
    let _params = scan_fmt_some!(b, &rust_fmt, i32);

    log::warn!("sscanf: stub, not fully implemented");
}

fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let source_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    if source_ptr == 0 {
        emu.maps.write_qword(dest_ptr, 0);
        log_red!(emu, "ntdll!RtlInitUnicodeString (null source)");
    } else {
        let s = emu.maps.read_wide_string(source_ptr);
        let byte_len = (s.len() * 2) as u16;
        emu.maps.write_word(dest_ptr, byte_len);
        emu.maps
            .write_word(dest_ptr + 2, byte_len.saturating_add(2));
        emu.maps.write_dword(dest_ptr + 4, source_ptr as u32);
        log_red!(emu, "ntdll!RtlInitUnicodeString `{}` len: {}", s, byte_len);
    }
    call.cleanup_args(emu, 2);
}

fn RtlInitAnsiString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let source_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    // ANSI_STRING32: u16 Length, u16 MaximumLength, u32 Buffer
    if source_ptr == 0 {
        emu.maps.write_qword(dest_ptr, 0);
        log_red!(emu, "ntdll!RtlInitAnsiString (null source)");
    } else {
        let s = emu.maps.read_string(source_ptr);
        let length = s.len() as u16;
        emu.maps.write_word(dest_ptr, length);
        emu.maps.write_word(dest_ptr + 2, length.saturating_add(1));
        emu.maps.write_dword(dest_ptr + 4, source_ptr as u32);
        log_red!(emu, "ntdll!RtlInitAnsiString `{}` len: {}", s, length);
    }
    call.cleanup_args(emu, 2);
}
