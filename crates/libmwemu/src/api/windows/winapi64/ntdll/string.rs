use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::api::dispatch_api;
use crate::api::windows::common::ntdll;
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "sscanf" => sscanf(emu),
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
        "RtlInitAnsiString" => RtlInitAnsiString(emu),
        "strcat" => strcat(emu),
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
}

fn strlen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0)) as usize;
    log_red!(emu, "** {} ntdll!strlen {:x}", emu.pos, s_ptr);

    if s_ptr == 0 {
        call.set_return(emu, 0);
        return;
    }

    let s = emu.maps.read_string(s_ptr as u64);
    let l = s.len();

    log_red!(emu, "ntdll!strlen: `{}` {}", s, l);

    call.set_return(emu, l as u32 as u64);
}

fn sscanf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buffer_ptr = api_try!(emu, call.arg(emu, 0));
    let fmt_ptr = api_try!(emu, call.arg(emu, 1));
    let _list = api_try!(emu, call.arg(emu, 2));

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
    let _params = rust_fmt.matches("{").count();

    log::warn!("sscanf: stub, not fully implemented");
}

pub fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_ptr = api_try!(emu, call.arg(emu, 0));
    let source_ptr = api_try!(emu, call.arg(emu, 1));

    if source_ptr == 0 {
        // Null source: zero out the UNICODE_STRING64 (16 bytes)
        emu.maps.write_qword(dest_ptr, 0);
        emu.maps.write_qword(dest_ptr + 8, 0);

        log_red!(emu, "ntdll!RtlInitUnicodeString (null source)");
    } else {
        let s = emu.maps.read_wide_string(source_ptr);
        let byte_length = (s.encode_utf16().count() * 2) as u16;

        // UNICODE_STRING64: u16 Length, u16 MaximumLength, u32 padding, u64 Buffer
        emu.maps.write_word(dest_ptr, byte_length);
        emu.maps
            .write_word(dest_ptr + 2, byte_length.saturating_add(2));
        emu.maps.write_dword(dest_ptr + 4, 0); // padding
        emu.maps.write_qword(dest_ptr + 8, source_ptr);

        log_red!(
            emu,
            "ntdll!RtlInitUnicodeString `{}` len: {}",
            s,
            byte_length
        );
    }

    call.set_return(emu, 0);
}

pub fn RtlInitAnsiString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_ptr = api_try!(emu, call.arg(emu, 0));
    let source_ptr = api_try!(emu, call.arg(emu, 1));

    if source_ptr == 0 {
        emu.maps.write_qword(dest_ptr, 0);
        emu.maps.write_qword(dest_ptr + 8, 0);

        log_red!(emu, "ntdll!RtlInitAnsiString (null source)");
    } else {
        let s = emu.maps.read_string(source_ptr);
        let length = s.len() as u16;

        // ANSI_STRING64: u16 Length, u16 MaximumLength, u32 padding, u64 Buffer
        emu.maps.write_word(dest_ptr, length);
        emu.maps.write_word(dest_ptr + 2, length.saturating_add(1));
        emu.maps.write_dword(dest_ptr + 4, 0); // padding
        emu.maps.write_qword(dest_ptr + 8, source_ptr);

        log_red!(emu, "ntdll!RtlInitAnsiString `{}` len: {}", s, length);
    }

    call.set_return(emu, 0);
}

fn strcat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    ntdll::strcat(emu, dst, src);
    call.set_return(emu, dst);
}
