use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::emu;
use crate::exception::types::ExceptionType;
use crate::kernel::windows::structs::NT_BUILD_NUMBER;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_array, read_guest_bytes, read_pointer};
use crate::winapi::common::{heap, memory, security};
use crate::winapi::policy;
use crate::winapi::policy::api_try;
use crate::winapi::registry;
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::structures::{OsVersionInfoA, OsVersionInfoExA, OsVersionInfoExW};

// Counted strings reserve one code unit for the terminator in a USHORT capacity.
const ANSI_STRING_MAX_BYTES: u16 = u16::MAX;
const UNICODE_STRING_MAX_BYTES: u16 = u16::MAX - 1;
const STRING_LENGTH_OFFSET: usize = 0;
const STRING_CAPACITY_OFFSET: usize = 2;
const STRING_BUFFER_OFFSET_32: usize = 4;
const STRING_BUFFER_OFFSET_64: usize = 8;
const STRING_HEADER_SIZE_32: usize = 8;
const STRING_HEADER_SIZE_64: usize = 16;

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

pub(crate) fn memset(emu: &mut emu::Emu, ptr: u64, byte: u64, count: u64) {
    log_red!(
        emu,
        "ntdll!memset ptr=0x{:x} byte={} count={}",
        ptr,
        byte,
        count
    );
    emu.maps.memset(ptr, byte as u8, count as usize);
}

pub(crate) fn memcpy(emu: &mut emu::Emu, dst: u64, src: u64, sz: usize) -> bool {
    log_red!(emu, "ntdll!memcpy=0x{:x} <- 0x{:x} {}", dst, src, sz);
    emu.maps.memcpy(dst, src, sz)
}

pub(crate) fn stricmp(emu: &mut emu::Emu, str1_ptr: u64, str2_ptr: u64) {
    let call = WinApi(Stdcall);
    let str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "ntdll!stricmp  '{}'=='{}'?", str1, str2);

    call.set_return(emu, if str1 == str2 { 0 } else { 1 });
}

const COMPRESSION_ENGINE_MASK: u16 = 0xFF00;

pub(crate) fn lznt1_decompress(input: &[u8], max_output: usize) -> Option<Vec<u8>> {
    if input.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(max_output.min(65536));
    let mut pos = 0;

    while pos + 2 <= input.len() {
        let header = u16::from_le_bytes([input[pos], input[pos + 1]]);
        pos += 2;

        if header == 0 {
            break;
        }

        let chunk_len = (header & 0x0FFF) as usize + 1;
        let is_compressed = (header & 0x8000) != 0;

        if pos + chunk_len > input.len() {
            return None;
        }

        if !is_compressed {
            if out.len() + chunk_len > max_output {
                return None;
            }
            out.extend_from_slice(&input[pos..pos + chunk_len]);
            pos += chunk_len;
            continue;
        }

        let chunk_end = pos + chunk_len;
        let chunk_start_out = out.len();

        while pos < chunk_end && out.len() < max_output {
            if pos >= input.len() {
                return None;
            }
            let flags = input[pos];
            pos += 1;

            for bit in 0..8u8 {
                if pos >= chunk_end || out.len() >= max_output {
                    break;
                }

                if (flags >> bit) & 1 == 0 {
                    out.push(input[pos]);
                    pos += 1;
                } else {
                    if pos + 2 > input.len() {
                        return None;
                    }
                    let tag = u16::from_le_bytes([input[pos], input[pos + 1]]);
                    pos += 2;

                    let cur_pos = out.len() - chunk_start_out;
                    let shift = if cur_pos <= 0x10 {
                        12
                    } else if cur_pos <= 0x20 {
                        11
                    } else if cur_pos <= 0x40 {
                        10
                    } else if cur_pos <= 0x80 {
                        9
                    } else if cur_pos <= 0x100 {
                        8
                    } else if cur_pos <= 0x200 {
                        7
                    } else if cur_pos <= 0x400 {
                        6
                    } else if cur_pos <= 0x800 {
                        5
                    } else {
                        4
                    };

                    let len_mask = (1u16 << shift) - 1;
                    let length = ((tag & len_mask) + 3) as usize;
                    let offset = ((tag >> shift) + 1) as usize;

                    if offset > out.len() - chunk_start_out {
                        return None;
                    }

                    for _ in 0..length {
                        if out.len() >= max_output {
                            break;
                        }
                        let src_idx = out.len() - offset;
                        let b = out[src_idx];
                        out.push(b);
                    }
                }
            }
        }

        pos = chunk_end;
    }

    Some(out)
}

pub(crate) fn xpress_decompress(input: &[u8], max_output: usize) -> Option<Vec<u8>> {
    if input.len() < 4 {
        return None;
    }
    let mut out = Vec::with_capacity(max_output.min(65536));
    let mut pos = 0;

    while pos < input.len() && out.len() < max_output {
        if pos + 4 > input.len() {
            break;
        }
        let indicator =
            u32::from_le_bytes([input[pos], input[pos + 1], input[pos + 2], input[pos + 3]]);
        pos += 4;

        for bit_idx in 0..32 {
            if pos >= input.len() || out.len() >= max_output {
                break;
            }
            if (indicator >> bit_idx) & 1 == 0 {
                out.push(input[pos]);
                pos += 1;
            } else {
                if pos + 2 > input.len() {
                    return Some(out);
                }
                let hint = u16::from_le_bytes([input[pos], input[pos + 1]]);
                pos += 2;

                let offset = ((hint >> 3) as usize) + 1;
                let mut length = (hint & 7) as usize;

                if length == 7 {
                    if pos >= input.len() {
                        return Some(out);
                    }
                    let extra = input[pos] as usize;
                    pos += 1;
                    if extra == 255 {
                        if pos + 2 > input.len() {
                            return Some(out);
                        }
                        let len16 = u16::from_le_bytes([input[pos], input[pos + 1]]) as usize;
                        pos += 2;
                        if len16 == 0 {
                            if pos + 4 > input.len() {
                                return Some(out);
                            }
                            length = u32::from_le_bytes([
                                input[pos],
                                input[pos + 1],
                                input[pos + 2],
                                input[pos + 3],
                            ]) as usize;
                            pos += 4;
                        } else {
                            length = len16;
                        }
                    } else {
                        length = extra + 7;
                    }
                }
                length += 3;

                if offset > out.len() {
                    return None;
                }
                for i in 0..length {
                    if out.len() >= max_output {
                        break;
                    }
                    let src = out.len() - offset;
                    let b = out[src + (i % offset)];
                    out.push(b);
                }
            }
        }
    }
    Some(out)
}

pub(crate) fn xpress_huff_decompress(input: &[u8], max_output: usize) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(max_output.min(65536));
    let mut src_pos = 0;

    while src_pos < input.len() && out.len() < max_output {
        if src_pos + 256 > input.len() {
            break;
        }

        let table_data = &input[src_pos..src_pos + 256];
        src_pos += 256;

        let mut lens = [0u8; 512];
        for i in 0..256 {
            lens[i * 2] = table_data[i] & 0x0F;
            lens[i * 2 + 1] = (table_data[i] >> 4) & 0x0F;
        }

        let mut decode_table = [0u16; 1 << 15];
        let mut code = 0u32;
        for bit_len in 1..=15u8 {
            for sym in 0..512u16 {
                if lens[sym as usize] != bit_len {
                    continue;
                }
                let fill_count = 1u32 << (15 - bit_len);
                let reversed = (code as u16).reverse_bits() >> (16 - bit_len);
                let base = (reversed as u32) << (15 - bit_len);
                for j in 0..fill_count {
                    let idx = (base + j) as usize;
                    if idx < decode_table.len() {
                        decode_table[idx] = sym | ((bit_len as u16) << 9);
                    }
                }
                code += 1;
            }
            code <<= 1;
        }

        let chunk_end = (src_pos + 65536).min(input.len());
        let chunk_data = &input[src_pos..chunk_end];
        let chunk_out_limit = (out.len() + 65536).min(max_output);

        if chunk_data.is_empty() {
            break;
        }

        let mut bit_buf: u32 = 0;
        let mut bits_left: u32 = 0;
        let mut cpos = 0;

        let refill = |buf: &mut u32, left: &mut u32, data: &[u8], p: &mut usize| {
            while *left < 16 && *p + 1 < data.len() {
                *buf |= (u16::from_le_bytes([data[*p], data[*p + 1]]) as u32) << *left;
                *left += 16;
                *p += 2;
            }
        };

        refill(&mut bit_buf, &mut bits_left, chunk_data, &mut cpos);

        while out.len() < chunk_out_limit {
            if bits_left < 15 {
                refill(&mut bit_buf, &mut bits_left, chunk_data, &mut cpos);
                if bits_left == 0 {
                    break;
                }
            }

            let peek = (bit_buf & 0x7FFF) as usize;
            let entry = decode_table[peek];
            let sym = entry & 0x1FF;
            let nbits = (entry >> 9) as u32;

            if nbits == 0 {
                break;
            }

            bit_buf >>= nbits;
            bits_left -= nbits;

            if sym < 256 {
                out.push(sym as u8);
            } else {
                let match_sym = sym - 256;
                let offset_log = (match_sym >> 4) as u32;
                let mut length = (match_sym & 0xF) as usize;

                if offset_log == 0 {
                    break;
                }

                let extra_offset_bits = offset_log - 1;
                if bits_left < extra_offset_bits {
                    refill(&mut bit_buf, &mut bits_left, chunk_data, &mut cpos);
                }
                let offset = if extra_offset_bits > 0 {
                    let extra = (bit_buf & ((1 << extra_offset_bits) - 1)) as usize;
                    bit_buf >>= extra_offset_bits;
                    bits_left -= extra_offset_bits;
                    (1usize << extra_offset_bits) + extra
                } else {
                    1
                };

                if length == 15 {
                    if cpos >= chunk_data.len() {
                        break;
                    }
                    let extra = chunk_data[cpos] as usize;
                    cpos += 1;
                    if extra == 255 {
                        if cpos + 2 > chunk_data.len() {
                            break;
                        }
                        let len16 =
                            u16::from_le_bytes([chunk_data[cpos], chunk_data[cpos + 1]]) as usize;
                        cpos += 2;
                        if len16 == 0 {
                            if cpos + 4 > chunk_data.len() {
                                break;
                            }
                            length = u32::from_le_bytes([
                                chunk_data[cpos],
                                chunk_data[cpos + 1],
                                chunk_data[cpos + 2],
                                chunk_data[cpos + 3],
                            ]) as usize;
                            cpos += 4;
                        } else {
                            length = len16;
                        }
                    } else {
                        length = extra + 15;
                    }
                }
                length += 3;

                if offset > out.len() || offset == 0 {
                    break;
                }
                for i in 0..length {
                    if out.len() >= chunk_out_limit {
                        break;
                    }
                    let src = out.len() - offset;
                    let b = out[src + (i % offset)];
                    out.push(b);
                }
            }
        }

        src_pos = chunk_end;
    }

    if out.is_empty() { None } else { Some(out) }
}

pub(crate) fn rtl_decompress_buffer(
    emu: &mut emu::Emu,
    compression_format: u16,
    uncompressed_buffer: u64,
    uncompressed_buffer_size: u64,
    compressed_buffer: u64,
    compressed_buffer_size: u64,
    final_uncompressed_size_ptr: u64,
) -> u64 {
    let fmt = compression_format & !COMPRESSION_ENGINE_MASK;

    if compressed_buffer_size > emu.maps.max_alloc_size
        || uncompressed_buffer_size > emu.maps.max_alloc_size
    {
        return STATUS_NO_MEMORY;
    }
    if final_uncompressed_size_ptr == 0
        || !emu
            .maps
            .validate_write_range(final_uncompressed_size_ptr, 4)
        || !emu
            .maps
            .validate_write_range(uncompressed_buffer, uncompressed_buffer_size)
    {
        return STATUS_ACCESS_VIOLATION;
    }
    let Some(compressed_data) =
        read_guest_bytes(emu, compressed_buffer, compressed_buffer_size as usize)
    else {
        return STATUS_ACCESS_VIOLATION;
    };

    let result = match fmt {
        COMPRESSION_FORMAT_LZNT1 => {
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer LZNT1 src=0x{:x}({}) dst=0x{:x}({})",
                compressed_buffer,
                compressed_buffer_size,
                uncompressed_buffer,
                uncompressed_buffer_size
            );
            lznt1_decompress(&compressed_data, uncompressed_buffer_size as usize)
        }
        COMPRESSION_FORMAT_XPRESS => {
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer Xpress src=0x{:x}({}) dst=0x{:x}({})",
                compressed_buffer,
                compressed_buffer_size,
                uncompressed_buffer,
                uncompressed_buffer_size
            );
            xpress_decompress(&compressed_data, uncompressed_buffer_size as usize)
        }
        COMPRESSION_FORMAT_XPRESS_HUFF => {
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer XpressHuff src=0x{:x}({}) dst=0x{:x}({})",
                compressed_buffer,
                compressed_buffer_size,
                uncompressed_buffer,
                uncompressed_buffer_size
            );
            xpress_huff_decompress(&compressed_data, uncompressed_buffer_size as usize)
        }
        _ => {
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer unknown format 0x{:x}",
                compression_format
            );
            None
        }
    };

    match result {
        Some(decompressed) => {
            let written = decompressed.len().min(uncompressed_buffer_size as usize);
            if written != 0
                && !emu
                    .maps
                    .write_bytes(uncompressed_buffer, &decompressed[..written])
            {
                return STATUS_ACCESS_VIOLATION;
            }
            emu.maps
                .write_dword(final_uncompressed_size_ptr, written as u32);
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer decompressed {} -> {} bytes",
                compressed_buffer_size,
                written
            );
            STATUS_SUCCESS
        }
        None => {
            if final_uncompressed_size_ptr != 0 {
                emu.maps.write_dword(final_uncompressed_size_ptr, 0);
            }
            STATUS_BAD_COMPRESSION_BUFFER
        }
    }
}

pub(crate) fn version_condition_mask(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    const VER_CONDITION_MASK: u64 = 7;
    const VER_NUM_BITS_PER_CONDITION: u32 = 3;
    let mut mask = api_try!(emu, call.arg_u64(emu, 0));
    let type_index = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
    let type_mask = api_try!(emu, call.arg_u32(emu, type_index));
    let condition = api_try!(emu, call.arg(emu, type_index + 1)) & VER_CONDITION_MASK;
    // The documented contract selects one OSVERSIONINFOEX member per call.
    if type_mask != 0 && condition != 0 {
        let member = type_mask.trailing_zeros();
        if member < u8::BITS {
            mask |= condition << (member * VER_NUM_BITS_PER_CONDITION);
        }
    }
    log_red!(
        emu,
        "VerSetConditionMask type=0x{:x} condition={} => 0x{:x}",
        type_mask,
        condition,
        mask
    );
    call.set_return_u64(emu, mask);
    call.cleanup_args(emu, 4);
}

fn counted_string_length(emu: &emu::Emu, source: u64, wide: bool) -> Option<(u16, bool)> {
    let unit_size = if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    };
    let capacity = if wide {
        UNICODE_STRING_MAX_BYTES
    } else {
        ANSI_STRING_MAX_BYTES
    };
    let limit = capacity as usize - unit_size;
    let mut length = 0;
    loop {
        let address = source.checked_add(length as u64)?;
        let unit = if wide {
            u16::from_le_bytes(read_array(emu, address)?)
        } else {
            emu.maps.read_byte(address)? as u16
        };
        if unit == 0 {
            return Some((length as u16, false));
        }
        if length == limit {
            return Some((limit as u16, true));
        }
        length += unit_size;
    }
}

fn initialize_counted_string(emu: &mut emu::Emu, name: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let (length, too_long) = if source == 0 {
        (0, false)
    } else if let Some(length) = counted_string_length(emu, source, wide) {
        length
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    let status = if extended && too_long {
        STATUS_NAME_TOO_LONG
    } else {
        STATUS_SUCCESS
    };
    let mut header = [0; STRING_HEADER_SIZE_64];
    let (header_size, buffer_offset) = if emu.cfg.arch.is_64bits() {
        (STRING_HEADER_SIZE_64, STRING_BUFFER_OFFSET_64)
    } else {
        (STRING_HEADER_SIZE_32, STRING_BUFFER_OFFSET_32)
    };
    // Ex leaves a zero-length descriptor pointing at the source when it is too long.
    if source != 0 && status == STATUS_SUCCESS {
        let capacity = length + if wide { 2 } else { 1 };
        header[STRING_LENGTH_OFFSET..STRING_LENGTH_OFFSET + size_of::<u16>()]
            .copy_from_slice(&length.to_le_bytes());
        header[STRING_CAPACITY_OFFSET..STRING_CAPACITY_OFFSET + size_of::<u16>()]
            .copy_from_slice(&capacity.to_le_bytes());
    }
    let pointer_size = header_size - buffer_offset;
    header[buffer_offset..header_size].copy_from_slice(&source.to_le_bytes()[..pointer_size]);
    if destination == 0 || !emu.maps.write_bytes(destination, &header[..header_size]) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    log_red!(
        emu,
        "{} destination=0x{:x} source=0x{:x} length={} status=0x{:x}",
        name,
        destination,
        source,
        length,
        status
    );
    if extended {
        call.set_return(emu, status);
    }
    call.cleanup_args(emu, 2);
}

pub(crate) fn format_current_user_key_path(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let output = api_try!(emu, call.arg(emu, 0));
    let width = memory::pointer_size(emu);
    let text = format!(
        r"\REGISTRY\USER\{}",
        security::sid_string(&security::process_user_sid())
    );
    let bytes: Vec<_> = text
        .encode_utf16()
        .chain(Some(0))
        .flat_map(u16::to_le_bytes)
        .collect();
    let capacity = bytes.len() as u16;
    let status = if output == 0 || !emu.maps.validate_write_range(output, 2 * width) {
        STATUS_ACCESS_VIOLATION
    } else if let Some(buffer) = heap::allocate(emu, 0, capacity as u64, 0) {
        let mut descriptor = [0; STRING_HEADER_SIZE_64];
        descriptor[0..2].copy_from_slice(&(capacity - 2).to_le_bytes());
        descriptor[2..4].copy_from_slice(&capacity.to_le_bytes());
        descriptor[width as usize..2 * width as usize]
            .copy_from_slice(&buffer.to_le_bytes()[..width as usize]);
        if emu.maps.write_bytes(buffer, &bytes)
            && emu
                .maps
                .write_bytes(output, &descriptor[..2 * width as usize])
        {
            STATUS_SUCCESS
        } else {
            heap::heap_free(emu, 0, buffer);
            STATUS_ACCESS_VIOLATION
        }
    } else {
        STATUS_NO_MEMORY
    };
    log_red!(
        emu,
        "ntdll!RtlFormatCurrentUserKeyPath output=0x{:x}",
        output
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub(crate) fn free_counted_string(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let descriptor = api_try!(emu, call.arg(emu, 0));
    let width = memory::pointer_size(emu);
    if descriptor != 0 {
        let buffer = descriptor
            .checked_add(width)
            .and_then(|p| memory::read_pointer(emu, p));
        if let Some(buffer) = buffer
            && emu.maps.validate_write_range(descriptor, 2 * width)
        {
            if buffer != 0 {
                heap::heap_free(emu, 0, buffer);
            }
            emu.maps.write_bytes(
                descriptor,
                &[0; STRING_HEADER_SIZE_64][..2 * width as usize],
            );
        } else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
    }
    log_red!(emu, "ntdll!{api} descriptor=0x{:x}", descriptor);
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch_counted_string(api: &str, library: &str, emu: &mut emu::Emu) -> bool {
    let (wide, extended) = match api {
        "RtlInitUnicodeString" => (true, false),
        "RtlInitUnicodeStringEx" => (true, true),
        "RtlInitAnsiString" | "RtlInitString" | "RtlInitUTF8String" => (false, false),
        "RtlInitAnsiStringEx" => (false, true),
        _ => return false,
    };
    initialize_counted_string(emu, &format!("{library}!{api}"), wide, extended);
    true
}

pub(crate) fn dispatch_user_profile(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlFormatCurrentUserKeyPath" => format_current_user_key_path(emu),
        "RtlFreeUnicodeString" | "RtlFreeAnsiString" => free_counted_string(emu, api),
        "RtlOpenCurrentUser" => registry::open_current_user(emu, true),
        _ => return false,
    }
    true
}

pub(crate) fn read_unicode_string(emu: &emu::Emu, descriptor: u64) -> Result<String, u64> {
    let width = pointer_size(emu);
    if descriptor == 0
        || !emu
            .maps
            .can_access_range(descriptor, (width * 2) as usize, Permission::READ)
    {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let len = emu
        .maps
        .read_word(descriptor)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    let max = emu
        .maps
        .read_word(descriptor + 2)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    if !len.is_multiple_of(2) || len > max {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if len == 0 {
        return Ok(String::new());
    }
    let buf = read_pointer(emu, descriptor + width).ok_or(STATUS_ACCESS_VIOLATION)?;
    if buf == 0 {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let bytes = read_guest_bytes(emu, buf, len as usize).ok_or(STATUS_ACCESS_VIOLATION)?;
    let words = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .copied()
        .map(u16::from_le_bytes)
        .collect::<Vec<_>>();
    String::from_utf16(&words).map_err(|_| STATUS_OBJECT_NAME_INVALID)
}

fn build_number(emu: &emu::Emu) -> u32 {
    emu.cfg
        .winver
        .as_deref()
        .and_then(|resolved| resolved.split('.').next()?.parse::<u32>().ok())
        .unwrap_or(u32::from(NT_BUILD_NUMBER))
}

pub(crate) fn packed_version(emu: &mut emu::Emu, library: &str) {
    let call = WinApi(Stdcall);
    let build = build_number(emu) as u16;
    let version = (u32::from(build) << u16::BITS) | 10;
    log_red!(emu, "{library}!GetVersion => {version:#x}");
    call.set_return(emu, u64::from(version));
}

pub(crate) fn version_info(emu: &mut emu::Emu, api: &str) {
    const VERSION_HEADER: usize = 5 * size_of::<u32>();
    const SERVICE_PACK_UNITS: usize = 128;
    const EXTENDED_FIELDS: usize = 8;
    const VER_PLATFORM_WIN32_NT: u32 = 2;
    const VER_NT_WORKSTATION: u8 = 1;
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let native = api == "RtlGetVersion";
    let width = if api == "GetVersionExA" { 1 } else { 2 };
    let basic = if width == 1 {
        OsVersionInfoA::SIZE as usize
    } else {
        OsVersionInfoExW::SIZE as usize - EXTENDED_FIELDS
    };
    let extended = if width == 1 {
        OsVersionInfoExA::SIZE
    } else {
        OsVersionInfoExW::SIZE
    } as usize;
    let result = (|| {
        let size = emu.maps.read_dword(out).ok_or(STATUS_ACCESS_VIOLATION)? as usize;
        if size != basic && size != extended {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if !emu.maps.validate_write_range(out, size as u64) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let build = build_number(emu);
        let mut info = [0; VERSION_HEADER + 2 * SERVICE_PACK_UNITS + EXTENDED_FIELDS];
        for (field, value) in
            info.chunks_exact_mut(4)
                .zip([size as u32, 10, 0, build, VER_PLATFORM_WIN32_NT])
        {
            field.copy_from_slice(&value.to_le_bytes());
        }
        if size > basic {
            info[basic + 6] = VER_NT_WORKSTATION;
        }
        emu.maps
            .write_bytes(out, &info[..size])
            .then_some(())
            .ok_or(STATUS_ACCESS_VIOLATION)
    })();
    log_red!(emu, "{api} output={out:#x} result={result:?}");
    let value = if native {
        result.err().unwrap_or(STATUS_SUCCESS)
    } else {
        if let Err(status) = result {
            emu.set_last_error(if status == STATUS_ACCESS_VIOLATION {
                ERROR_NOACCESS
            } else {
                ERROR_INSUFFICIENT_BUFFER
            });
        }
        u64::from(result.is_ok())
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 1);
}

pub fn event_enabled(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let registration = api_try!(emu, call.arg_u64(emu, 0));
    let descriptor_index = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
    let descriptor = api_try!(emu, call.arg(emu, descriptor_index));
    log_red!(
        emu,
        "EventEnabled registration={registration:#x} descriptor={descriptor:#x}"
    );
    // The guest has no enabled trace sessions.
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
pub(crate) fn etw(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "EtwEventEnabled" => event_enabled(emu),
        "EtwEventRegister"
        | "EtwEventUnregister"
        | "EtwEventSetInformation"
        | "EtwEventWriteTransfer" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        _ => return false,
    }
    true
}
pub(crate) fn byte_string(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = if api == "strcat" {
        Some(api_try!(emu, call.arg(emu, 1)))
    } else {
        None
    };
    let result = (|| {
        let count = |at: u64| -> Option<u64> {
            if at == 0 {
                return None;
            }
            let mut end = at;
            while emu.maps.read_byte(end)? != 0 {
                end = end.checked_add(1)?;
            }
            Some(end - at)
        };
        let length = count(dst)?;
        if let Some(src) = src {
            let bytes =
                read_guest_bytes(emu, src, usize::try_from(count(src)?.checked_add(1)?).ok()?)?;
            let end = dst.checked_add(length)?;
            if !emu.maps.validate_write_range(end, bytes.len() as u64)
                || !emu.maps.write_bytes(end, &bytes)
            {
                return None;
            }
            Some(dst)
        } else {
            Some(length)
        }
    })();
    log_red!(emu, "ntdll!{api} buffer={dst:#x}");
    if let Some(value) = result {
        call.set_return(emu, value);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub fn RtlDecompressBuffer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let format = api_try!(emu, call.arg_u16(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2));
    let source = api_try!(emu, call.arg(emu, 3));
    let length = api_try!(emu, call.arg_u32(emu, 4));
    let final_size = api_try!(emu, call.arg(emu, 5));
    let status = rtl_decompress_buffer(
        emu,
        format,
        out,
        u64::from(capacity),
        source,
        u64::from(length),
        final_size,
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 6);
}
pub(crate) fn pointer_round_trip(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    // The guest model preserves opaque pointer round trips without a process cookie.
    log_red!(emu, "{api} pointer={ptr:#x}");
    call.set_return(emu, ptr);
    call.cleanup_args(emu, 1);
}
pub(crate) fn restore_last_error(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let error = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "RtlRestoreLastWin32Error error={error}");
    emu.set_last_error(u64::from(error));
    call.cleanup_args(emu, 1);
}
pub(crate) fn native_tick(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "NtGetTickCount tick={}", emu.tick);
    call.set_return(emu, u64::from(emu.tick as u32));
}
pub(crate) fn set_exception_filter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filter = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "RtlSetUnhandledExceptionFilter filter={filter:#x}");
    emu.set_uef(filter);
    call.cleanup_args(emu, 1);
}

pub fn GetVersionExA(emu: &mut emu::Emu) {
    version_info(emu, "GetVersionExA");
}

pub fn GetVersionExW(emu: &mut emu::Emu) {
    version_info(emu, "GetVersionExW");
}

pub fn EncodePointer(emu: &mut emu::Emu) {
    pointer_round_trip(emu, "EncodePointer");
}

pub fn DecodePointer(emu: &mut emu::Emu) {
    pointer_round_trip(emu, "DecodePointer");
}
