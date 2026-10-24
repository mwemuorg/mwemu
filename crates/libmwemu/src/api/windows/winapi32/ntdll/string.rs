use crate::api::windows::common::ntdll;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use scan_fmt::scan_fmt_some;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "strcat" => strcat(emu),
        "sscanf" => sscanf(emu),
        "RtlInitAnsiString" => RtlInitAnsiString(emu),
        _ => return false,
    }
    true
}

fn stricmp(emu: &mut emu::Emu) {
    let str1ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let str2ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    ntdll::stricmp(emu, str1ptr, str2ptr);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let len = ntdll::strlen(emu, s_ptr);
    emu.stack_pop32(false);
    emu.regs_mut().rax = len as u32 as u64;
}

fn strcat(emu: &mut emu::Emu) {
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    let dst_cont_ptr = ntdll::strcat(emu, dst_ptr, src_ptr);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = dst_cont_ptr;
}

fn sscanf(emu: &mut emu::Emu) {
    let buffer_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let list = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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

fn RtlInitAnsiString(emu: &mut emu::Emu) {
    let dest_ptr = arg(emu, 0) as u64;
    let source_ptr = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
