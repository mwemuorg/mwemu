use crate::api::windows::common::ntdll;
use crate::emu;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "sscanf" => sscanf(emu),
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
        "RtlInitAnsiString" => RtlInitAnsiString(emu),
        _ => return false,
    }
    true
}

fn stricmp(emu: &mut emu::Emu) {
    let str1ptr = emu.regs().rcx;
    let str2ptr = emu.regs().rdx;
    ntdll::stricmp(emu, str1ptr, str2ptr);
}

fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx as usize;
    log_red!(emu, "** {} ntdll!strlen {:x}", emu.pos, s_ptr);

    if s_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let s = emu.maps.read_string(s_ptr as u64);
    let l = s.len();

    log_red!(emu, "ntdll!strlen: `{}` {}", s, l);

    emu.regs_mut().rax = l as u32 as u64;
}

fn sscanf(emu: &mut emu::Emu) {
    let buffer_ptr = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let _list = emu.regs().r8;

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

    unimplemented!("sscanf is unimplemented for now.");
}

pub fn RtlInitUnicodeString(emu: &mut emu::Emu) {
    let dest_ptr = emu.regs().rcx;
    let source_ptr = emu.regs().rdx;

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

    emu.regs_mut().rax = 0;
}

pub fn RtlInitAnsiString(emu: &mut emu::Emu) {
    let dest_ptr = emu.regs().rcx;
    let source_ptr = emu.regs().rdx;

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

    emu.regs_mut().rax = 0;
}
