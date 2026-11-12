use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;

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
        {
            log::warn!("RtlMoveMemory failed to copy");
        }
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

    let compressed_data = emu
        .maps
        .read_bytes(compressed_buffer, compressed_buffer_size as usize);

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
            emu.maps
                .write_bytes(uncompressed_buffer, &decompressed[..written]);
            if final_uncompressed_size_ptr != 0 {
                emu.maps
                    .write_dword(final_uncompressed_size_ptr, written as u32);
            }
            log_red!(
                emu,
                "ntdll!RtlDecompressBuffer decompressed {} -> {} bytes",
                compressed_buffer_size,
                written
            );
            0 // STATUS_SUCCESS
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
    const VER_CONDITION_MASK: u64 = 7;
    const VER_NUM_BITS_PER_CONDITION: u32 = 3;
    let mut mask = WinApiAbi::arg64(emu, 0);
    let type_index = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
    let type_mask = WinApiAbi::arg(emu, type_index) as u32;
    let condition = WinApiAbi::arg(emu, type_index + 1) & VER_CONDITION_MASK;
    // The documented contract selects one OSVERSIONINFOEX member per call.
    if type_mask != 0 && condition != 0 {
        let member = type_mask.trailing_zeros();
        if member < u8::BITS {
            mask |= condition << (member * VER_NUM_BITS_PER_CONDITION);
        }
    }
    log_red!(
        emu,
        "VerSetConditionMask type:0x{:x} condition:{} => 0x{:x}",
        type_mask,
        condition,
        mask
    );
    WinApiAbi::set_ret64(emu, mask);
    WinApiAbi::stdcall_ret(emu, 4);
}
