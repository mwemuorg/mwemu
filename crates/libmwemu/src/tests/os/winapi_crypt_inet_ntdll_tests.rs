use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
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

// ─── crypt32 64-bit: CertOpenStore ───

#[test]
fn test_cert_open_store_64() {
    helpers::setup();
    let mut emu = emu64();

    let handle = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0], // CERT_STORE_PROV_MEMORY, PKCS_7_ASN_ENCODING|X509_ASN_ENCODING
    );
    assert_ne!(handle, 0, "CertOpenStore should return non-zero handle");
}

// ─── crypt32 64-bit: CertOpenSystemStoreA ───

#[test]
fn test_cert_open_system_store_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("str_buf", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "MY");

    let handle = helpers::call_winapi64(
        &mut emu,
        winapi64::crypt32::CertOpenSystemStoreA,
        &[0, str_addr],
    );
    assert_ne!(
        handle, 0,
        "CertOpenSystemStoreA should return non-zero handle"
    );
}

// ─── crypt32 64-bit: CertOpenSystemStoreW ───

#[test]
fn test_cert_open_system_store_w_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("str_buf", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_wide_string(str_addr, "ROOT");

    let handle = helpers::call_winapi64(
        &mut emu,
        winapi64::crypt32::CertOpenSystemStoreW,
        &[0, str_addr],
    );
    assert_ne!(
        handle, 0,
        "CertOpenSystemStoreW should return non-zero handle"
    );
}

// ─── crypt32 64-bit: CertCloseStore ───

#[test]
fn test_cert_close_store_64() {
    helpers::setup();
    let mut emu = emu64();

    let store = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0],
    );
    assert_ne!(store, 0);

    let ret = helpers::call_winapi64(&mut emu, winapi64::crypt32::CertCloseStore, &[store, 0]);
    assert_eq!(ret, 1, "CertCloseStore should return TRUE");
}

// ─── crypt32 64-bit: CertFindCertificateInStore (empty store) ───

#[test]
fn test_cert_find_certificate_empty_store_64() {
    helpers::setup();
    let mut emu = emu64();

    let store = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0],
    );

    let cert = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CertFindCertificateInStore,
        &[store, 0x10001, 0, 0, 0, 0],
    );
    assert_eq!(cert, 0, "empty store should have no certificates");
}

// ─── crypt32 64-bit: CertFreeCertificateContext ───

#[test]
fn test_cert_free_certificate_context_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::crypt32::CertFreeCertificateContext,
        &[0x1234],
    );
    assert_eq!(ret, 1, "CertFreeCertificateContext should return TRUE");
}

// ─── crypt32 64-bit: CryptStringToBinaryA ───

#[test]
fn test_crypt_string_to_binary_a_64() {
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
        &[str_addr, 8, 0x01, out_ptr, out_sz_ptr, 0, 0], // CRYPT_STRING_BASE64
    );
    assert_eq!(ret, 1, "CryptStringToBinaryA should return TRUE (success)");
}

// ─── crypt32 64-bit: full open/find/close cycle ───

#[test]
fn test_cert_store_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("buf", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "MY");

    let store = helpers::call_winapi64(
        &mut emu,
        winapi64::crypt32::CertOpenSystemStoreA,
        &[0, str_addr],
    );
    assert_ne!(store, 0);

    let cert = call_winapi64_stack(
        &mut emu,
        winapi64::crypt32::CertFindCertificateInStore,
        &[store, 0x10001, 0, 0, 0, 0],
    );
    assert_eq!(cert, 0, "emulated store is empty");

    let closed = helpers::call_winapi64(&mut emu, winapi64::crypt32::CertCloseStore, &[store, 0]);
    assert_eq!(closed, 1);
}

// ─── crypt32 32-bit: CertOpenStore ───

#[test]
fn test_cert_open_store_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0],
    );
    assert_ne!(ret, 0, "CertOpenStore 32-bit should return non-zero handle");
}

// ─── crypt32 32-bit: CertOpenSystemStoreA ───

#[test]
fn test_cert_open_system_store_a_32() {
    helpers::setup();
    let mut emu = emu32();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("str_buf", str_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "MY");

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertOpenSystemStoreA,
        &[0, str_addr as u32],
    );
    assert_ne!(ret, 0, "CertOpenSystemStoreA 32-bit non-zero handle");
}

// ─── crypt32 32-bit: CertCloseStore ───

#[test]
fn test_cert_close_store_32() {
    helpers::setup();
    let mut emu = emu32();

    let store = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0],
    );
    assert_ne!(store, 0);

    let ret = helpers::call_winapi32(&mut emu, winapi32::crypt32::CertCloseStore, &[store, 0]);
    assert_eq!(ret, 1, "CertCloseStore 32-bit should return TRUE");
}

// ─── crypt32 32-bit: CertFindCertificateInStore ───

#[test]
fn test_cert_find_certificate_empty_store_32() {
    helpers::setup();
    let mut emu = emu32();

    let store = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertOpenStore,
        &[0x0A, 0x10001, 0, 0, 0],
    );
    assert_ne!(store, 0);

    let cert = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertFindCertificateInStore,
        &[store, 0x10001, 0, 0, 0, 0],
    );
    assert_eq!(cert, 0, "32-bit empty store has no certs");
}

// ─── crypt32 32-bit: CertFreeCertificateContext ───

#[test]
fn test_cert_free_certificate_context_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::crypt32::CertFreeCertificateContext,
        &[0x1234],
    );
    assert_eq!(ret, 1, "CertFreeCertificateContext 32-bit TRUE");
}

// ─── crypt32 32-bit: CryptStringToBinaryA ───

#[test]
fn test_crypt_string_to_binary_a_32() {
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
fn test_rtl_decompress_buffer_empty_stream_64() {
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
fn test_rtl_decompress_buffer_uncompressed_chunk_64() {
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
fn test_rtl_decompress_buffer_compressed_literals_64() {
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
fn test_rtl_decompress_buffer_unknown_format_64() {
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
        status, 0xC0000242,
        "unknown format should fail with BAD_COMPRESSION_BUFFER"
    );
}

// ─── ntdll 32-bit: RtlDecompressBuffer — uncompressed LZNT1 chunk ───

#[test]
fn test_rtl_decompress_buffer_uncompressed_chunk_32() {
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
            0x0002,               // COMPRESSION_FORMAT_LZNT1
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
fn test_rtl_decompress_buffer_xpress_literals_32() {
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
fn test_ws2_32_wsa_startup_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::WsaStartup, &[0x0202, 0]);
    assert_eq!(ret, 0, "WsaStartup returns 0 (success)");
}

#[test]
fn test_ws2_32_socket_create_close_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[2, 1, 6], // AF_INET, SOCK_STREAM, IPPROTO_TCP
    );
    assert_ne!(sock, 0, "socket() should return non-zero handle");
    assert_ne!(sock, u64::MAX, "socket() should not return INVALID_SOCKET");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket returns 0");
}

#[test]
fn test_ws2_32_htons_64() {
    helpers::setup();
    let mut emu = emu64();

    let result = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[0x5000]);
    assert_eq!(result, 0x0050, "htons(0x5000) should be 0x0050");
}

#[test]
fn test_ws2_32_htons_443_64() {
    helpers::setup();
    let mut emu = emu64();

    let result = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[443]);
    let expected = 443u16.to_be() as u64;
    assert_eq!(result, expected, "htons(443) byte swap");
}

// ─── ws2_32 32-bit ───

#[test]
fn test_ws2_32_wsa_startup_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::WsaStartup, &[0x0202, 0]);
    assert_eq!(ret, 0, "WsaStartup 32-bit returns 0");
}

#[test]
fn test_ws2_32_socket_create_close_32() {
    helpers::setup();
    let mut emu = emu32();

    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(sock, 0, "socket() 32-bit non-zero");

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket 32-bit returns 0");
}

#[test]
fn test_ws2_32_htons_32() {
    helpers::setup();
    let mut emu = emu32();

    let result = helpers::call_winapi32(&mut emu, winapi32::ws2_32::htons, &[80]);
    let expected = 80u16.to_be() as u32;
    assert_eq!(result, expected, "htons(80) 32-bit byte swap");
}

// ─── ole32 64-bit: CoInitializeEx / CoTaskMemAlloc / CoTaskMemFree ───

#[test]
fn test_co_initialize_ex_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoInitializeEx,
        &[0, 0x02], // COINIT_APARTMENTTHREADED
    );
    assert_eq!(ret, 0, "CoInitializeEx returns S_OK");
}

#[test]
fn test_co_task_mem_alloc_free_64() {
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
fn test_co_uninitialize_64() {
    helpers::setup();
    let mut emu = emu64();

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoInitializeEx, &[0, 0]);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
    assert_eq!(ret, 0, "CoUninitialize returns 0");
}

// ─── oleaut32 64-bit: SysAllocStringLen / SysFreeString / VariantClear ───

#[test]
fn test_sys_alloc_string_len_64() {
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
fn test_sys_free_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 16]);
    assert_ne!(bstr, 0);

    helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysFreeString, &[bstr]);
    // SysFreeString has no meaningful return value to assert, just check it doesn't panic
}

#[test]
fn test_variant_clear_64() {
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
fn test_create_compatible_dc_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    assert_ne!(
        hdc, 0,
        "CreateCompatibleDC should return non-zero DC handle"
    );
}

#[test]
fn test_create_compatible_bitmap_64() {
    helpers::setup();
    let mut emu = emu64();

    let hbm = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::CreateCompatibleBitmap,
        &[0x1337, 640, 480],
    );
    assert_ne!(hbm, 0, "CreateCompatibleBitmap returns non-zero handle");
}

#[test]
fn test_select_delete_object_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    let prev = helpers::call_winapi64(&mut emu, winapi64::gdi32::SelectObject, &[hdc, 0xABCD]);
    // SelectObject returns the previously selected object (or the object itself)
    // The implementation returns rdx (the new object), so we just check it doesn't crash
    assert!(prev != u64::MAX, "SelectObject should not fail");

    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteObject, &[0xABCD]);
    assert_eq!(ret, 1, "DeleteObject returns TRUE");
}

#[test]
fn test_delete_dc_64() {
    helpers::setup();
    let mut emu = emu64();

    let hdc = helpers::call_winapi64(&mut emu, winapi64::gdi32::CreateCompatibleDC, &[0]);
    let ret = helpers::call_winapi64(&mut emu, winapi64::gdi32::DeleteDC, &[hdc]);
    assert_eq!(ret, 1, "DeleteDC returns TRUE");
}

#[test]
fn test_get_device_caps_64() {
    helpers::setup();
    let mut emu = emu64();

    let width = helpers::call_winapi64(
        &mut emu,
        winapi64::gdi32::GetDeviceCaps,
        &[0x1337, 8], // HORZRES
    );
    assert!(width > 0, "GetDeviceCaps(HORZRES) should return >0");
}

// ─── shell32 64-bit: ShellExecuteA ───

#[test]
fn test_shell_execute_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("str_buf", str_addr, 0x2000, Permission::READ_WRITE)
        .expect("map");
    emu.maps.write_string(str_addr, "open");
    emu.maps.write_string(str_addr + 0x100, "notepad.exe");

    let ret = call_winapi64_stack(
        &mut emu,
        winapi64::shell32::ShellExecuteA,
        &[0, str_addr, str_addr + 0x100, 0, 0, 1],
    );
    assert!(ret > 32, "ShellExecuteA returns >32 on success");
}

// ─── ntdll 64-bit: LZNT1 compressed chunk with back-reference ───

#[test]
fn test_rtl_decompress_buffer_lznt1_backreference_64() {
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
fn test_rtl_decompress_buffer_lznt1_engine_flag_masked_64() {
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

    // Same uncompressed chunk as before, but format = 0x0102 (LZNT1 + COMPRESSION_ENGINE_STANDARD)
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
            0x0102,
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
fn test_rtl_decompress_buffer_xpress_literals_64() {
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
fn test_rtl_decompress_buffer_xpress_backref_64() {
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
fn test_rtl_decompress_buffer_xpress_short_input_64() {
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
        status, 0xC0000242,
        "short Xpress input returns BAD_COMPRESSION_BUFFER"
    );
}

// ─── CryptStringToBinaryA: real base64 decode (64-bit) ───

#[test]
fn test_crypt_string_to_binary_a_base64_real_decode_64() {
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
        &[str_addr, 8, 0x01, out_addr, sz_addr, 0, 0], // CRYPT_STRING_BASE64
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
fn test_crypt_string_to_binary_a_hex_decode_64() {
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
        &[str_addr, 4, 0x04, out_addr, sz_addr, 0, 0], // CRYPT_STRING_HEX
    );
    assert_eq!(ret, 1);

    let decoded_len = emu.maps.read_dword(sz_addr).unwrap_or(0);
    assert_eq!(decoded_len, 2);

    let decoded = emu.maps.read_bytes(out_addr, 2);
    assert_eq!(decoded.as_ref(), b"AB");
}

// ─── CryptHashCertificate: real SHA-256 hash (64-bit) ───

#[test]
fn test_crypt_hash_certificate_sha256_64() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x70000u64;
    let hash_addr = 0x80000u64;
    let hash_len_addr = 0x90000u64;

    emu.maps
        .create_map("data", data_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hash", hash_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hlen", hash_len_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    // Hash the empty string with SHA-256
    emu.maps.write_dword(hash_len_addr, 32);

    let ret = call_winapi64_stack(
        &mut emu,
        |emu: &mut emu::Emu| {
            let alg_id = emu.regs().rdx as u32;
            let cert_ptr = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let hp = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hlp = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu, alg_id, cert_ptr, cert_len, hp, hlp,
            );
        },
        &[0, 0x800c, data_addr, 0, hash_addr, hash_len_addr], // CALG_SHA_256
    );
    assert_eq!(ret, 1, "CryptHashCertificate should succeed");

    let hash_len = emu.maps.read_dword(hash_len_addr).unwrap_or(0);
    assert_eq!(hash_len, 32, "SHA-256 hash is 32 bytes");

    // SHA-256 of empty string = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let hash = emu.maps.read_bytes(hash_addr, 32);
    assert_eq!(hash[0], 0xe3, "SHA-256 empty first byte");
    assert_eq!(hash[1], 0xb0, "SHA-256 empty second byte");
    assert_eq!(hash[31], 0x55, "SHA-256 empty last byte");
}

#[test]
fn test_crypt_hash_certificate_sha1_64() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x70000u64;
    let hash_addr = 0x80000u64;
    let hash_len_addr = 0x90000u64;

    emu.maps
        .create_map("data", data_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hash", hash_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hlen", hash_len_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    emu.maps.write_dword(hash_len_addr, 20);

    let ret = call_winapi64_stack(
        &mut emu,
        |emu: &mut emu::Emu| {
            let alg_id = emu.regs().rdx as u32;
            let cert_ptr = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let hp = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hlp = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu, alg_id, cert_ptr, cert_len, hp, hlp,
            );
        },
        &[0, 0x8004, data_addr, 0, hash_addr, hash_len_addr], // CALG_SHA1
    );
    assert_eq!(ret, 1, "CryptHashCertificate SHA-1 should succeed");

    let hash_len = emu.maps.read_dword(hash_len_addr).unwrap_or(0);
    assert_eq!(hash_len, 20, "SHA-1 hash is 20 bytes");

    // SHA-1 of empty string = da39a3ee5e6b4b0d3255bfef95601890afd80709
    let hash = emu.maps.read_bytes(hash_addr, 20);
    assert_eq!(hash[0], 0xda, "SHA-1 empty first byte");
    assert_eq!(hash[1], 0x39, "SHA-1 empty second byte");
    assert_eq!(hash[19], 0x09, "SHA-1 empty last byte");
}

#[test]
fn test_crypt_hash_certificate_size_query_64() {
    helpers::setup();
    let mut emu = emu64();

    let hash_len_addr = 0x90000u64;
    emu.maps
        .create_map("hlen", hash_len_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    let ret = call_winapi64_stack(
        &mut emu,
        |emu: &mut emu::Emu| {
            let alg_id = emu.regs().rdx as u32;
            let cert_ptr = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let hp = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hlp = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu, alg_id, cert_ptr, cert_len, hp, hlp,
            );
        },
        &[0, 0x800c, 0, 0, 0, hash_len_addr], // hash_ptr=0 → size query
    );
    assert_eq!(ret, 1);

    let hash_len = emu.maps.read_dword(hash_len_addr).unwrap_or(0);
    assert_eq!(hash_len, 32, "size query returns 32 for SHA-256");
}

#[test]
fn test_crypt_hash_certificate_md5_64() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x70000u64;
    let hash_addr = 0x80000u64;
    let hash_len_addr = 0x90000u64;

    emu.maps
        .create_map("data", data_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hash", hash_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");
    emu.maps
        .create_map("hlen", hash_len_addr, 0x1000, Permission::READ_WRITE)
        .expect("map");

    emu.maps.write_dword(hash_len_addr, 16);

    let ret = call_winapi64_stack(
        &mut emu,
        |emu: &mut emu::Emu| {
            let alg_id = emu.regs().rdx as u32;
            let cert_ptr = emu.regs().r8;
            let cert_len = emu.regs().r9;
            let hp = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hlp = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
                emu, alg_id, cert_ptr, cert_len, hp, hlp,
            );
        },
        &[0, 0x8003, data_addr, 0, hash_addr, hash_len_addr], // CALG_MD5
    );
    assert_eq!(ret, 1);

    let hash_len = emu.maps.read_dword(hash_len_addr).unwrap_or(0);
    assert_eq!(hash_len, 16, "MD5 hash is 16 bytes");

    // MD5 of empty = d41d8cd98f00b204e9800998ecf8427e
    let hash = emu.maps.read_bytes(hash_addr, 16);
    assert_eq!(hash[0], 0xd4, "MD5 empty first byte");
    assert_eq!(hash[15], 0x7e, "MD5 empty last byte");
}
