use crate::emu;

const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_WRITECOPY: u32 = 0x08;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

pub(crate) fn protect_to_rwx(protect_value: u32) -> (bool, bool, bool) {
    let can_read = (protect_value
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_write = (protect_value
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_execute = (protect_value
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    (can_read, can_write, can_execute)
}

pub(crate) fn rtl_zero_memory(emu: &mut emu::Emu, dest: u64, length: u64) {
    log_red!(
        emu,
        "ntdll!RtlZeroMemory dest: 0x{:x} length: {}",
        dest,
        length
    );
    emu.maps.memset(dest, 0, length as usize);
}

pub(crate) fn rtl_move_memory(emu: &mut emu::Emu, dst: u64, src: u64, sz: usize) {
    log_red!(
        emu,
        "ntdll!RtlMoveMemory dst = {:x} src = {:x} sz = {}",
        dst,
        src,
        sz
    );

    if !emu.maps.memcpy(dst, src, sz) {
        panic!("RtlMoveMemory failed to copy");
    }
}

pub(crate) fn memset(emu: &mut emu::Emu, ptr: u64, byte: u64, count: u64) {
    log_red!(
        emu,
        "ntdll!memset ptr: 0x{:x} byte: {} count: {}",
        ptr,
        byte,
        count
    );
    emu.maps.memset(ptr, byte as u8, count as usize);
}

pub(crate) fn memcpy(emu: &mut emu::Emu, dst: u64, src: u64, sz: usize) -> bool {
    log_red!(emu, "ntdll!memcpy: 0x{:x} <- 0x{:x} {}", dst, src, sz);
    emu.maps.memcpy(dst, src, sz)
}

pub(crate) fn strlen(emu: &mut emu::Emu, ptr: u64) -> u64 {
    if ptr == 0 {
        return 0;
    }

    let s = emu.maps.read_string(ptr);
    let len = s.len() as u64;
    log_red!(emu, "ntdll!strlen: `{}` {}", s, len);
    len
}

pub(crate) fn strcat(emu: &mut emu::Emu, dst: u64, src: u64) -> u64 {
    let dst_str = emu.maps.read_string(dst);
    let src_str = emu.maps.read_string(src);

    log_red!(emu, "ntdll!strcat: `{}`+`{}`", src_str, dst_str);

    let dst_cont_ptr = dst + dst_str.len() as u64;
    emu.maps.write_string(dst_cont_ptr, &src_str);
    dst_cont_ptr
}

pub(crate) fn stricmp(emu: &mut emu::Emu, str1_ptr: u64, str2_ptr: u64) {
    let str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "ntdll!stricmp  '{}'=='{}'?", str1, str2);

    emu.regs_mut().rax = if str1 == str2 { 0 } else { 1 };
}
