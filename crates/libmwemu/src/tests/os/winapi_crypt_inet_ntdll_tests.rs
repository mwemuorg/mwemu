use crate::api::windows::policy::ApiMode;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::libraries::gdi32;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;
use crate::*;

/// Call a 64-bit WinAPI that reads stack args at rsp+0x28, rsp+0x30, ...
/// (the standard x64 convention with shadow space). Args 0-3 go in rcx/rdx/r8/r9,
/// args 4+ go at rsp+0x28+((i-4)*8).
fn call_winapi64_stack(emu: &mut emu::Emu, func: fn(&mut emu::Emu), args: &[u64]) -> u64 {
    // Ensure stack via a no-op call
    let _ = helpers::call_winapi64(emu, |e| e.regs_mut().rax = 0, &[]);
    let setters: [fn(&mut emu::Emu, u64); 4] = [
        |e, v| e.regs_mut().rcx = v,
        |e, v| e.regs_mut().rdx = v,
        |e, v| e.regs_mut().r8 = v,
        |e, v| e.regs_mut().r9 = v,
    ];
    for (i, &a) in args.iter().take(4).enumerate() {
        setters[i](emu, a);
    }
    let rsp = emu.regs().rsp;
    for (i, &a) in args.iter().enumerate().skip(4) {
        emu.maps.write_qword(rsp + 0x20 + ((i - 4) as u64) * 8, a);
    }
    func(emu);
    emu.regs().rax
}

// ─── crypt32 64-bit: CryptStringToBinaryA ───

#[test]
fn crypt_string_to_binary_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("buf", str_addr, 0x2000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "SGVsbG8="); // "Hello" in base64

    let out_ptr = str_addr + 0x100;
    let out_sz_ptr = str_addr + 0x200;
    emu.maps.write_dword(out_sz_ptr, 256);

    let ret = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CryptStringToBinaryA,
        &[str_addr, 8, CRYPT_STRING_BASE64, out_ptr, out_sz_ptr, 0, 0],
    );
    assert_eq!(ret, 1, "CryptStringToBinaryA should return TRUE (success)");
}

// ─── crypt32 32-bit: CryptStringToBinaryA ───

#[test]
fn crypt_string_to_binary_a_32() {
    helpers::setup();
    let mut emu = emu32();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("buf", str_addr, 0x2000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "SGVsbG8=");

    let out_ptr = (str_addr + 0x100) as u32;
    let out_sz_ptr = (str_addr + 0x200) as u32;
    emu.maps.write_dword(out_sz_ptr as u64, 256);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CryptStringToBinaryA,
        &[str_addr as u32, 8, 0x01, out_ptr, out_sz_ptr, 0, 0],
    );
    assert_eq!(ret, 1, "CryptStringToBinaryA 32-bit TRUE");
}

// ─── ntdll 64-bit: RtlDecompressBuffer — empty LZNT1 stream ───

#[test]
fn rtl_decompress_buffer_empty_stream_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Empty LZNT1: just the terminator (two zero bytes)
    emu.maps.write_byte(comp_addr, 0x00);
    emu.maps.write_byte(comp_addr + 1, 0x00);

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[0x0002, decomp_addr, 0x1000, comp_addr, 2, final_sz_addr],
    );
    assert_eq!(status, 0, "STATUS_SUCCESS for empty LZNT1 stream");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0xFFFF);
    assert_eq!(final_sz, 0, "empty stream decompresses to 0 bytes");
}

// ─── ntdll 64-bit: RtlDecompressBuffer — uncompressed LZNT1 chunk ───

#[test]
fn rtl_decompress_buffer_uncompressed_chunk_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Uncompressed LZNT1 chunk: header bit15=0, low12 = payload_len - 1
    // 5 bytes "Hello" → header = 0x0004 (5-1=4)
    let data: &[u8] = &[
        0x04, 0x00, // chunk header: uncompressed, 5 bytes
        b'H', b'e', b'l', b'l', b'o', // payload
        0x00, 0x00, // terminator
    ];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            0x0002,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "STATUS_SUCCESS");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 5, "decompressed size should be 5");

    let out = emu.maps.read_bytes(decomp_addr, 5);
    assert_eq!(out.as_ref(), b"Hello", "decompressed data mismatch");
}

// ─── ntdll 64-bit: RtlDecompressBuffer — compressed LZNT1 chunk with all literals ───

#[test]
fn rtl_decompress_buffer_compressed_literals_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Compressed chunk with flag byte 0x00 (all 8 bits literal) + 8 literal bytes
    // chunk header: bit15=1 (compressed), low12 = payload_len - 1 = 9 - 1 = 8
    // So header = 0x8008
    let data: &[u8] = &[
        0x08, 0x80, // chunk header: compressed, 9 bytes payload
        0x00, // flags: all 8 are literals
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', // 8 literals
        0x00, 0x00, // terminator
    ];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            0x0002,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "STATUS_SUCCESS for compressed literals");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 8, "8 literal bytes");

    let out = emu.maps.read_bytes(decomp_addr, 8);
    assert_eq!(out.as_ref(), b"ABCDEFGH");
}

// ─── ntdll 64-bit: RtlDecompressBuffer — unknown format ───

#[test]
fn rtl_decompress_buffer_unknown_format_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Unknown format 0x00FF
    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[0x00FF, decomp_addr, 0x1000, comp_addr, 2, final_sz_addr],
    );
    assert_eq!(
        status, STATUS_BAD_COMPRESSION_BUFFER,
        "unknown format should fail with BAD_COMPRESSION_BUFFER"
    );
}

// ─── ntdll 32-bit: RtlDecompressBuffer — uncompressed LZNT1 chunk ───

#[test]
fn rtl_decompress_buffer_uncompressed_chunk_32() {
    helpers::setup();
    let mut emu = emu32();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Uncompressed LZNT1: "ABCD" (4 bytes), header = 0x0003
    let data: &[u8] = &[
        0x03, 0x00, // header: uncompressed, 4 bytes
        b'A', b'B', b'C', b'D', 0x00, 0x00, // terminator
    ];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlDecompressBuffer,
        &[
            COMPRESSION_FORMAT_LZNT1 as u32,
            decomp_addr as u32,   // uncompressed buffer
            0x1000,               // uncompressed buffer size
            comp_addr as u32,     // compressed buffer
            data.len() as u32,    // compressed buffer size
            final_sz_addr as u32, // final uncompressed size ptr
        ],
    );
    assert_eq!(ret, 0, "STATUS_SUCCESS for 32-bit LZNT1");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 4, "decompressed 4 bytes");

    let out = emu.maps.read_bytes(decomp_addr, 4);
    assert_eq!(out.as_ref(), b"ABCD");
}

// ─── ntdll 32-bit: RtlDecompressBuffer — XPRESS fails ───

#[test]
fn rtl_decompress_buffer_xpress_literals_32() {
    helpers::setup();
    let mut emu = emu32();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Xpress plain: indicator=0 (all literals), then "Hi!"
    let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, b'H', b'i', b'!'];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlDecompressBuffer,
        &[
            0x0003,
            decomp_addr as u32,
            0x1000,
            comp_addr as u32,
            data.len() as u32,
            final_sz_addr as u32,
        ],
    );
    assert_eq!(ret, 0, "Xpress 32-bit literals should succeed");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 3, "decompressed 3 bytes");

    let out = emu.maps.read_bytes(decomp_addr, 3);
    assert_eq!(out.as_ref(), b"Hi!");
}

// ─── ws2_32 64-bit: socket lifecycle ───

#[test]
fn ws2_32_wsa_startup_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::WSAStartup, &[0x0202, 0]);
    assert_eq!(ret, WSAEFAULT, "WSAStartup requires writable WSADATA");
}

#[test]
fn ws2_32_htons_64() {
    helpers::setup();
    let mut emu = emu64();

    let result = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[0x5000]);
    assert_eq!(result, 0x0050, "htons(0x5000) should be 0x0050");
}

#[test]
fn ws2_32_htons_443_64() {
    helpers::setup();
    let mut emu = emu64();

    let result = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[443]);
    let expected = 443u16.to_be() as u64;
    assert_eq!(result, expected, "htons(443) byte swap");
}

// ─── ws2_32 32-bit ───

#[test]
fn ws2_32_wsa_startup_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::WSAStartup, &[0x0202, 0]);
    assert_eq!(
        ret as u64, WSAEFAULT,
        "WSAStartup requires writable WSADATA"
    );
}

#[test]
fn ws2_32_htons_32() {
    helpers::setup();
    let mut emu = emu32();

    let result = helpers::call_winapi32(&mut emu, winapi32::ws2_32::htons, &[80]);
    let expected = 80u16.to_be() as u32;
    assert_eq!(result, expected, "htons(80) 32-bit byte swap");
}

// ─── ole32 64-bit: CoInitializeEx / CoTaskMemAlloc / CoTaskMemFree ───

#[test]
fn co_initialize_ex_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoInitializeEx,
        &[0, COINIT_APARTMENTTHREADED as u64],
    );
    assert_eq!(ret, 0, "CoInitializeEx returns S_OK");
}

#[test]
fn co_task_mem_alloc_free_64() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "CoTaskMemAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xCAFEBABE);

    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);
    assert_eq!(ret, 0, "CoTaskMemFree returns 0");
}

#[test]
fn co_uninitialize_64() {
    helpers::setup();
    let mut emu = emu64();

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoInitializeEx, &[0, 0]);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
    assert_eq!(ret, 0, "CoUninitialize returns 0");
}

// ─── oleaut32 64-bit: SysAllocStringLen / SysFreeString / VariantClear ───

#[test]
fn sys_alloc_string_len_64() {
    helpers::setup();
    let mut emu = emu64();

    let bstr = helpers::call_winapi64(
        &mut emu,
        winapi64::oleaut32::SysAllocStringLen,
        &[0, 32], // NULL source, 32 chars
    );
    assert_ne!(bstr, 0, "SysAllocStringLen returned NULL");
    assert!(emu.maps.is_mapped(bstr));
}

#[test]
fn sys_free_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 16]);
    assert_ne!(bstr, 0);

    helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysFreeString, &[bstr]);
    // SysFreeString has no meaningful return value to assert, just check it doesn't panic
}

#[test]
fn variant_clear_64() {
    helpers::setup();
    let mut emu = emu64();

    let variant_addr = 0x30000u64;
    emu.maps
        .create_map("variant", variant_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    // Write a VARIANT struct (16 bytes zeroed = VT_EMPTY)
    for i in 0..16u64 {
        emu.maps.write_byte(variant_addr + i, 0);
    }

    let ret = helpers::call_winapi64(&mut emu, winapi64::oleaut32::VariantClear, &[variant_addr]);
    assert_eq!(ret, 0, "VariantClear returns S_OK");
}

// ─── gdi32 64-bit: CreateCompatibleDC / SelectObject / DeleteDC ───

#[test]
fn create_compatible_dc_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(
        hdc, 0,
        "CreateCompatibleDC should return non-zero DC handle"
    );
}

#[test]
fn select_delete_object_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let prev = helpers::call_winapi64(&mut emu, gdi32::SelectObject, &[hdc, 0xABCD]);
    assert_eq!(prev, 0);

    let ret = helpers::call_winapi64(&mut emu, gdi32::DeleteObject, &[0xABCD]);
    assert_eq!(ret, 0, "DeleteObject rejects an unrelated handle");
}

#[test]
fn delete_dc_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi64(&mut emu, gdi32::DeleteDC, &[hdc]);
    assert_eq!(ret, 1, "DeleteDC returns TRUE");
}

// ─── ntdll 64-bit: LZNT1 compressed chunk with back-reference ───

#[test]
fn rtl_decompress_buffer_lznt1_backreference_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Build a compressed chunk that uses a back-reference.
    // We want to produce "AAAAAA" (6 x 'A'):
    // - First: 4 literal 'A's via flag byte 0x00 and 4 literal bytes
    //   (only process 4 bits since we only need 4 from this flag set)
    //   Actually, let's do: flag byte with bit0=0 (literal), bit1=1 (backreference)
    //   rest bits don't matter if we run out of chunk.
    //
    // At output position 1 within the chunk, shift=12, so:
    //   offset field = 4 bits (top), length field = 12 bits (bottom)
    //   tag: offset=1 (means go back 1 byte), length=3 (encoded as 3-3=0)
    //   So: offset_val = (1-1) = 0, shifted: 0 << 12 = 0
    //        length_val = 0 (means 3)
    //   tag = 0x0000
    //
    // Payload: flag=0x02 (bit1 set=backreference, bit0 clear=literal)
    //          literal byte 'A'
    //          backreference: 0x00 0x00 → offset=1, length=3 → copies 3 bytes
    //          total decompressed: 1 literal + 3 copy = 4 bytes "AAAA"
    // Chunk payload = 4 bytes (flag + literal + 2-byte tag)
    // Chunk header: 0x8003 (compressed, payload_len=4, i.e. 4-1=3)
    let data: &[u8] = &[
        0x03, 0x80, // chunk header: compressed, 4 bytes payload
        0x02, // flags: bit0=literal, bit1=backreference
        b'A', // literal 'A'
        0x00, 0x00, // backreference tag: offset=1, length=3
        0x00, 0x00, // terminator
    ];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            0x0002,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "STATUS_SUCCESS for backreference chunk");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 4, "1 literal + 3 backreference = 4 bytes");

    let out = emu.maps.read_bytes(decomp_addr, 4);
    assert_eq!(out.as_ref(), b"AAAA", "back-reference should repeat 'A'");
}

// ─── ntdll 64-bit: LZNT1 with ENGINE flag in format (should be masked) ───

#[test]
fn rtl_decompress_buffer_lznt1_engine_flag_masked_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Compression engine bits must not change the selected format.
    let data: &[u8] = &[
        0x02, 0x00, // header: uncompressed, 3 bytes
        b'X', b'Y', b'Z', 0x00, 0x00,
    ];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            (COMPRESSION_FORMAT_LZNT1 | COMPRESSION_ENGINE_MAXIMUM) as u64,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "engine flag should be masked, LZNT1 succeeds");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 3);

    let out = emu.maps.read_bytes(decomp_addr, 3);
    assert_eq!(out.as_ref(), b"XYZ");
}

// ─── Xpress plain: all-literals decompression (64-bit) ───

#[test]
fn rtl_decompress_buffer_xpress_literals_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Xpress plain: indicator=0 (all literals), then "ABCD"
    let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, b'A', b'B', b'C', b'D'];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            0x0003,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "Xpress all-literals should succeed");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 4, "decompressed 4 literal bytes");

    let out = emu.maps.read_bytes(decomp_addr, 4);
    assert_eq!(out.as_ref(), b"ABCD");
}

// ─── Xpress plain: back-reference decompression (64-bit) ───

#[test]
fn rtl_decompress_buffer_xpress_backref_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Xpress: literal 'A' then match(offset=1, length=3)
    // indicator = 0x00000002 (bit 1 set = match)
    // literal: 0x41 = 'A'
    // match hint: offset-1=0 shifted left 3, length-3=0 → u16 LE = 0x0000
    // Result: "AAAA"
    let data: &[u8] = &[0x02, 0x00, 0x00, 0x00, 0x41, 0x00, 0x00];
    for (i, &b) in data.iter().enumerate() {
        emu.maps.write_byte(comp_addr + i as u64, b);
    }

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[
            0x0003,
            decomp_addr,
            0x1000,
            comp_addr,
            data.len() as u64,
            final_sz_addr,
        ],
    );
    assert_eq!(status, 0, "Xpress backref should succeed");

    let final_sz = emu.maps.read_dword(final_sz_addr).unwrap_or(0);
    assert_eq!(final_sz, 4, "1 literal + 3 copy = 4 bytes");

    let out = emu.maps.read_bytes(decomp_addr, 4);
    assert_eq!(out.as_ref(), b"AAAA");
}

// ─── Xpress plain: short input returns error (64-bit) ───

#[test]
fn rtl_decompress_buffer_xpress_short_input_64() {
    helpers::setup();
    let mut emu = emu64();

    let comp_addr = 0x40000u64;
    let decomp_addr = 0x50000u64;
    let final_sz_addr = 0x60000u64;

    emu.maps
        .create_map("comp", comp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("decomp", decomp_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("final_sz", final_sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Only 2 bytes — too short for even the indicator
    emu.maps.write_byte(comp_addr, 0x00);
    emu.maps.write_byte(comp_addr + 1, 0x00);

    let status = call_winapi64_stack(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[0x0003, decomp_addr, 0x1000, comp_addr, 2, final_sz_addr],
    );
    assert_eq!(
        status, STATUS_BAD_COMPRESSION_BUFFER,
        "short Xpress input returns BAD_COMPRESSION_BUFFER"
    );
}

// ─── CryptStringToBinaryA: real base64 decode (64-bit) ───

#[test]
fn crypt_string_to_binary_a_base64_real_decode_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x70000u64;
    let out_addr = 0x80000u64;
    let sz_addr = 0x90000u64;

    emu.maps
        .create_map("str", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("out", out_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("sz", sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // "SGVsbG8=" is base64 for "Hello"
    let b64 = b"SGVsbG8=\x00";
    emu.maps.write_bytes(str_addr, b64);
    emu.maps.write_dword(sz_addr, 256);

    let ret = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CryptStringToBinaryA,
        &[str_addr, 8, CRYPT_STRING_BASE64, out_addr, sz_addr, 0, 0],
    );
    assert_eq!(ret, 1, "CryptStringToBinaryA should succeed");

    let decoded_len = emu.maps.read_dword(sz_addr).unwrap_or(0);
    assert_eq!(decoded_len, 5, "decoded length should be 5");

    let decoded = emu.maps.read_bytes(out_addr, 5);
    assert_eq!(
        decoded.as_ref(),
        b"Hello",
        "decoded content should be 'Hello'"
    );
}

#[test]
fn crypt_string_to_binary_a_hex_decode_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x70000u64;
    let out_addr = 0x80000u64;
    let sz_addr = 0x90000u64;

    emu.maps
        .create_map("str", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("out", out_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("sz", sz_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // hex for "AB" = 0x41 0x42
    let hex = b"4142\x00";
    emu.maps.write_bytes(str_addr, hex);
    emu.maps.write_dword(sz_addr, 256);

    let ret = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CryptStringToBinaryA,
        &[str_addr, 4, CRYPT_STRING_HEX, out_addr, sz_addr, 0, 0],
    );
    assert_eq!(ret, 1);

    let decoded_len = emu.maps.read_dword(sz_addr).unwrap_or(0);
    assert_eq!(decoded_len, 2);

    let decoded = emu.maps.read_bytes(out_addr, 2);
    assert_eq!(decoded.as_ref(), b"AB");
}

#[test]
fn certificate_hashes_use_sdk_argument_slots_and_atomic_output() {
    for is64 in [false, true] {
        let (mut emu, bytes) = helpers::prepare_winapi(is64);
        let length = bytes + 64;
        let call = |emu: &mut _, id: u32, output| {
            helpers::call_winapi_export(
                emu,
                "crypt32.dll",
                "CryptHashCertificate",
                &[0, u64::from(id), 0, 0, 0, output, length],
            )
        };
        for (algorithm, size, first, last) in [
            (0, 20, 0xda, 0x09),
            (CALG_SHA1, 20, 0xda, 0x09),
            (CALG_SHA_256, 32, 0xe3, 0x55),
            (CALG_MD5, 16, 0xd4, 0x7e),
        ] {
            assert_eq!(call(&mut emu, algorithm, 0), 1);
            assert_eq!(emu.maps.read_dword(length), Some(size));
            emu.maps.fill_range(bytes, 0xcc, 40);
            emu.maps.write_dword(length, size - 1);
            assert_eq!(call(&mut emu, algorithm, bytes), 0);
            assert_eq!(emu.maps.read_byte(bytes), Some(0xcc));
            assert_eq!(call(&mut emu, algorithm, bytes), 1);
            assert_eq!(emu.maps.read_byte(bytes), Some(first));
            assert_eq!(emu.maps.read_byte(bytes + u64::from(size) - 1), Some(last));
            assert_eq!(emu.maps.read_byte(bytes + u64::from(size)), Some(0xcc));
        }
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "crypt32.dll",
                "CryptHashCertificate",
                &[0, 0, 0, 1, u64::from(u32::MAX), bytes, length]
            ),
            0
        );
    }
}

#[test]
fn winhttp_full_flow_64() {
    helpers::setup();
    let mut emu = emu64();

    let agent_addr = 0x100000u64;
    let server_addr = 0x100100u64;
    let verb_addr = 0x100200u64;
    let path_addr = 0x100300u64;
    let bytes_read_addr = 0x100400u64;
    let buffer_addr = 0x100500u64;
    emu.maps
        .create_map("http_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"TestAgent"
    let agent: [u8; 20] = [
        b'T', 0, b'e', 0, b's', 0, b't', 0, b'A', 0, b'g', 0, b'e', 0, b'n', 0, b't', 0, 0, 0,
    ];
    for (i, &b) in agent.iter().enumerate() {
        emu.maps.write_byte(agent_addr + i as u64, b);
    }
    // Write L"example.com"
    let server: [u8; 24] = [
        b'e', 0, b'x', 0, b'a', 0, b'm', 0, b'p', 0, b'l', 0, b'e', 0, b'.', 0, b'c', 0, b'o', 0,
        b'm', 0, 0, 0,
    ];
    for (i, &b) in server.iter().enumerate() {
        emu.maps.write_byte(server_addr + i as u64, b);
    }
    // Write L"GET"
    let get: [u8; 8] = [b'G', 0, b'E', 0, b'T', 0, 0, 0];
    for (i, &b) in get.iter().enumerate() {
        emu.maps.write_byte(verb_addr + i as u64, b);
    }
    // Write L"/api"
    let path: [u8; 10] = [b'/', 0, b'a', 0, b'p', 0, b'i', 0, 0, 0];
    for (i, &b) in path.iter().enumerate() {
        emu.maps.write_byte(path_addr + i as u64, b);
    }

    // WinHttpOpen(agent, accessType, proxy, bypass, flags)
    let h_session = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpOpen,
        &[agent_addr, 0, 0, 0, 0],
    );
    assert_ne!(h_session, 0, "WinHttpOpen must return a session handle");

    // WinHttpConnect(session, server, port, reserved)
    let h_connect = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpConnect,
        &[h_session, server_addr, 443, 0],
    );
    assert_ne!(
        h_connect, 0,
        "WinHttpConnect must return a connection handle"
    );

    // WinHttpOpenRequest(connect, verb, objectName, version, referrer, acceptTypes, flags)
    let h_request = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpOpenRequest,
        &[h_connect, verb_addr, path_addr, 0, 0, 0, 0x00800000],
    );
    assert_ne!(
        h_request, 0,
        "WinHttpOpenRequest must return a request handle"
    );

    emu.set_api_mode(ApiMode::SimulateFail);
    emu.maps.write_dword(bytes_read_addr, 0x12345678);
    for (handler, args) in [
        (
            winapi64::winhttp::WinHttpSendRequest as fn(&mut Emu),
            vec![h_request, 0, 0, 0, 0, 0, 0],
        ),
        (
            winapi64::winhttp::WinHttpReceiveResponse,
            vec![h_request, 0],
        ),
        (
            winapi64::winhttp::WinHttpReadData,
            vec![h_request, buffer_addr, 0x100, bytes_read_addr],
        ),
    ] {
        assert_eq!(helpers::call_winapi64(&mut emu, handler, &args), 0);
    }
    assert_eq!(emu.maps.read_dword(bytes_read_addr), Some(0x12345678));
    // WinHttpCloseHandle — close all in reverse order
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_request],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(request) must return TRUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_connect],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(connect) must return TRUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_session],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(session) must return TRUE");
}

#[test]
fn ntdll_restore_last_error_aliases_preserve_call_frame() {
    const NONVOLATILE_MARKER: u64 = 0x1234_5678;
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        for api in ["RtlRestoreLastWin32Error", "RestoreLastError"] {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            emu.regs_mut().rbx = NONVOLATILE_MARKER;
            helpers::call_winapi_export(&mut emu, "ntdll.dll", api, &[ERROR_ACCESS_DENIED]);
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            assert_eq!(emu.regs().rbx, NONVOLATILE_MARKER);
            assert!(emu.api_policy.last_outcome.is_none());
        }
    }
}

#[test]
fn internet_reads_do_not_invent_payloads_or_overwrite_buffers() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.set_api_mode(ApiMode::SimulateFail);
        for (dll, api, args) in [
            ("wininet.dll", "InternetReadFile", [1, data, 1, data + 8]),
            ("wininet.dll", "InternetReadFileExW", [1, data, 0, 0]),
            ("winhttp.dll", "WinHttpReadData", [1, data, 1, data + 8]),
        ] {
            emu.maps.write_bytes(data, &[0xa5; 16]);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, dll, api, &args),
                0,
                "{dll}!{api} is64={is64}"
            );
            assert_eq!(emu.maps.read_bytes(data, 16), vec![0xa5; 16]);
        }
    }
}

#[test]
fn rtl_critical_sections_share_ownership_and_return_native_status() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::{pointer_size, read_pointer};
    use crate::windows::constants::errors::{STATUS_ACCESS_VIOLATION, STATUS_SUCCESS};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let width = pointer_size(&emu);
        let size = 4 * width + 8;
        let tid = emu.current_thread().id;
        emu.maps.write_byte(data + size, 0xa5);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "RtlInitializeCriticalSectionEx",
                &[data, 19, 0]
            ),
            STATUS_SUCCESS
        );
        assert_eq!(read_pointer(&emu, data + size - width), Some(19));
        for recursion in [1, 2] {
            assert_eq!(
                call(&mut emu, "ntdll.dll", "RtlEnterCriticalSection", &[data]),
                STATUS_SUCCESS
            );
            assert_eq!(emu.maps.read_dword(data + width + 4), Some(recursion));
            assert_eq!(read_pointer(&emu, data + width + 8), Some(tid));
        }
        for recursion in [1, 0] {
            assert_eq!(
                call(&mut emu, "ntdll.dll", "RtlLeaveCriticalSection", &[data]),
                STATUS_SUCCESS
            );
            assert_eq!(emu.maps.read_dword(data + width + 4), Some(recursion));
        }
        assert_eq!(
            call(&mut emu, "ntdll.dll", "RtlDeleteCriticalSection", &[data]),
            STATUS_SUCCESS
        );
        assert_eq!(emu.maps.read_byte(data + size), Some(0xa5));
        assert_eq!(
            call(&mut emu, "ntdll.dll", "RtlInitializeCriticalSection", &[0]),
            STATUS_ACCESS_VIOLATION
        );
    }
}

#[test]
fn runtime_function_lookup_returns_a_guest_table_entry_for_an_image_pc() {
    use crate::maps::mem64::Permission;
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    let (mut emu, out) = prepare_winapi(true);
    let base = 0x4100_0000;
    let pe_offset = 0x80;
    let table_rva = 0x200;
    let entry = base + table_rva;
    emu.maps
        .create_map("unwind_fixture.pe", base, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(base + 0x3c, pe_offset as u32);
    let pe = base + pe_offset;
    emu.maps.write_dword(pe + 0x50, 0x1000);
    emu.maps.write_dword(pe + 0x84, 16);
    emu.maps.write_dword(pe + 0xa0, table_rva as u32);
    emu.maps.write_dword(pe + 0xa4, 12);
    emu.maps.write_dword(entry, 0x300);
    emu.maps.write_dword(entry + 4, 0x380);
    emu.maps.write_dword(entry + 8, 0x480);
    for dll in ["ntdll.dll", "kernel32.dll"] {
        emu.maps.write_qword(out, 0);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RtlLookupFunctionEntry",
                &[base + 0x37f, out, 0]
            ),
            entry
        );
        assert_eq!(emu.maps.read_qword(out), Some(base));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RtlLookupFunctionEntry",
                &[base + 0x380, out, 0]
            ),
            0
        );
    }
}

#[test]
fn internet_handles_keep_their_library_parent_and_request_url() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::{errors::*, network::*};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let (host, path, size, output) = (data, data + 64, data + 128, data + 256);
        emu.maps.write_string(host, "example.test");
        emu.maps.write_string(path, "/local");
        let inet = "wininet.dll";
        let session = call(&mut emu, inet, "InternetOpenA", &[0; 5]);
        assert_ne!(session, 0);
        let connection = call(
            &mut emu,
            inet,
            "InternetConnectA",
            &[
                session,
                host,
                8443,
                0,
                0,
                INTERNET_SERVICE_HTTP.into(),
                0,
                0,
            ],
        );
        let request = call(
            &mut emu,
            inet,
            "HttpOpenRequestA",
            &[connection, 0, path, 0, 0, 0, INTERNET_FLAG_SECURE, 0],
        );
        assert_ne!(connection, 0);
        assert_ne!(request, 0);
        assert_eq!(
            call(&mut emu, "winhttp.dll", "WinHttpCloseHandle", &[request]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        emu.maps.write_dword(size, 0);
        assert_eq!(
            call(
                &mut emu,
                inet,
                "InternetQueryOptionW",
                &[request, INTERNET_OPTION_URL.into(), 0, size]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        let expected = "https://example.test:8443/local";
        assert_eq!(
            emu.maps.read_dword(size),
            Some(((expected.len() + 1) * 2) as u32)
        );
        assert_eq!(
            call(
                &mut emu,
                inet,
                "InternetQueryOptionW",
                &[request, INTERNET_OPTION_URL.into(), output, size]
            ),
            1
        );
        assert_eq!(emu.maps.read_wide_string(output), expected);
        assert_eq!(emu.maps.read_dword(size), Some((expected.len() * 2) as u32));
        assert_eq!(
            call(
                &mut emu,
                inet,
                "InternetQueryOptionA",
                &[connection, INTERNET_OPTION_HANDLE_TYPE.into(), output, size]
            ),
            1
        );
        assert_eq!(
            emu.maps.read_dword(output),
            Some(INTERNET_HANDLE_TYPE_CONNECT_HTTP)
        );
        assert_eq!(call(&mut emu, inet, "InternetCloseHandle", &[session]), 1);
        for closed in [request, connection, session] {
            assert_eq!(call(&mut emu, inet, "InternetCloseHandle", &[closed]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
    }
}

#[test]
fn dns_compares_names_and_clears_offline_query_results() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::{read_pointer, write_pointer};
    use crate::windows::constants::errors::DNS_ERROR_RCODE_SERVER_FAILURE;
    for is64 in [false, true] {
        let (mut emu, first) = prepare_winapi(is64);
        let (second, records) = (first + 64, first + 128);
        emu.maps.write_wide_string(first, "Ä.Host..");
        emu.maps.write_wide_string(second, "ä.host");
        assert_eq!(
            call(&mut emu, "dnsapi.dll", "DnsNameCompare_W", &[first, second]),
            1
        );
        emu.maps.write_string(first, "server");
        emu.maps.write_string(second, "other.");
        assert_eq!(
            call(&mut emu, "dnsapi.dll", "DnsNameCompare_A", &[first, second]),
            0
        );
        write_pointer(&mut emu, records, 0x1234);
        assert_eq!(
            call(
                &mut emu,
                "dnsapi.dll",
                "DnsQuery_A",
                &[first, 1, 0, 0, records, 0]
            ),
            DNS_ERROR_RCODE_SERVER_FAILURE
        );
        assert_eq!(read_pointer(&emu, records), Some(0));
    }
}

#[test]
fn certificate_comparisons_use_signed_serials_issuers_and_guest_time() {
    use crate::winapi::common::memory::{pointer_size, write_pointer};
    for is64 in [false, true] {
        let (mut emu, base) = helpers::prepare_winapi(is64);
        let width = pointer_size(&emu);
        let other = base + 128;
        let bytes = base + 320;
        let time = bytes + 32;
        emu.maps.write_bytes(bytes, &[0xff, 0xff, 0, b'A', b'B']);
        for (info, serial_size, issuer) in [(base, 1, bytes + 3), (other, 2, bytes + 3)] {
            emu.maps.write_dword(info + width, serial_size);
            write_pointer(&mut emu, info + 2 * width, bytes);
            emu.maps.write_dword(info + 6 * width, 1);
            write_pointer(&mut emu, info + 7 * width, issuer);
            emu.maps.write_qword(info + 8 * width, 5);
            emu.maps.write_qword(info + 8 * width + 8, 15);
        }
        let call = |emu: &mut _, name: &str, args: &[u64]| {
            helpers::call_winapi_export(emu, "crypt32.dll", name, args)
        };
        assert_eq!(
            call(&mut emu, "CertCompareCertificate", &[1, base, other]),
            1
        );
        write_pointer(&mut emu, other + 7 * width, bytes + 4);
        assert_eq!(
            call(&mut emu, "CertCompareCertificate", &[1, base, other]),
            0
        );
        assert_eq!(
            call(&mut emu, "CertVerifyValidityNesting", &[base, other]),
            1
        );
        for (when, expected) in [(4, u64::from(u32::MAX)), (5, 0), (15, 0), (16, 1)] {
            emu.maps.write_qword(time, when);
            assert_eq!(
                call(&mut emu, "CertVerifyTimeValidity", &[time, base]),
                expected
            );
        }
    }
}

#[test]
fn wininet_cache_groups_and_local_urls_respect_guest_widths() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::ERROR_INSUFFICIENT_BUFFER;
    for is64 in [false, true] {
        let (mut emu, base) = prepare_winapi(is64);
        let lib = "wininet.dll";
        let group = call(&mut emu, lib, "CreateUrlCacheGroup", &[0, 0]);
        assert_ne!(group, 0);
        let args = if is64 {
            vec![group, 0, 0]
        } else {
            vec![group, 0, 0, 0]
        };
        assert_eq!(call(&mut emu, lib, "DeleteUrlCacheGroup", &args), 1);
        assert_eq!(call(&mut emu, lib, "DeleteUrlCacheGroup", &args), 0);
        let (child, out, length) = (base + 64, base + 128, base + 256);
        emu.maps.write_string(base, "http://example.test/a/b");
        emu.maps.write_string(child, "../c");
        emu.maps.write_dword(length, 1);
        assert_eq!(
            call(
                &mut emu,
                lib,
                "InternetCombineUrlA",
                &[base, child, out, length, 0]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(
            call(
                &mut emu,
                lib,
                "InternetCombineUrlA",
                &[base, child, out, length, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_string(out), "http://example.test/c");
    }
}

#[test]
fn empty_certificate_stores_follow_provider_and_handle_contracts() {
    use crate::windows::constants::crypto::{CERT_STORE_PROV_MEMORY, CERT_STORE_PROV_SYSTEM_W};
    use crate::windows::constants::errors::{
        CRYPT_E_NOT_FOUND, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER,
    };
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let dll = "crypt32.dll";
        assert_eq!(
            call(
                &mut emu,
                dll,
                "CertOpenStore",
                &[CERT_STORE_PROV_SYSTEM_W, 0, 0, 0, 0]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
        let memory_store = call(
            &mut emu,
            dll,
            "CertOpenStore",
            &[CERT_STORE_PROV_MEMORY, 0, 0, 0, 0],
        );
        assert_ne!(memory_store, 0);
        emu.maps.write_string(data, "MY");
        emu.maps.write_wide_string(data + 32, "ROOT");
        let ansi_store = call(&mut emu, dll, "CertOpenSystemStoreA", &[0, data]);
        let wide_store = call(&mut emu, dll, "CertOpenSystemStoreW", &[0, data + 32]);
        for store in [memory_store, ansi_store, wide_store] {
            assert_ne!(store, 0);
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "CertFindCertificateInStore",
                    &[store, 0, 0, 0, 0, 0]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), CRYPT_E_NOT_FOUND);
            assert_eq!(call(&mut emu, dll, "CertCloseStore", &[store, 0]), 1);
            assert_eq!(call(&mut emu, dll, "CertCloseStore", &[store, 0]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
        assert_eq!(call(&mut emu, dll, "CertFreeCertificateContext", &[0]), 1);
    }
}
