use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::*;

// ─── ws2_32 64-bit ───

#[test]
fn test_wsa_startup_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::WsaStartup, &[0x0202, 0]);
    assert_eq!(ret, 0, "WSAStartup should return 0 (success)");
}

#[test]
fn test_socket_create_64() {
    helpers::setup();
    let mut emu = emu64();
    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(sock, 0, "socket() should return a non-zero handle");
}

#[test]
fn test_htons_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[80]);
    assert_eq!(ret, 80u16.to_be() as u64, "htons(80) should byte-swap");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::htons, &[0x1234]);
    assert_eq!(
        ret,
        0x1234u16.to_be() as u64,
        "htons(0x1234) should byte-swap"
    );
}

#[test]
fn test_closesocket_64() {
    helpers::setup();
    let mut emu = emu64();
    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket should return 0");
}

#[test]
fn test_connect_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let sockaddr_base = 0x30000u64;
    emu.maps
        .create_map("sockaddr", sockaddr_base, 0x1000, Permission::READ_WRITE);
    // AF_INET=2, port=80 (big-endian=0x5000), ip=127.0.0.1
    emu.maps.write_word(sockaddr_base, 2); // family
    emu.maps.write_word(sockaddr_base + 2, 80u16.to_be()); // port
    emu.maps.write_dword(sockaddr_base + 4, 0x0100007f); // 127.0.0.1

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::connect,
        &[sock, sockaddr_base, 16],
    );
    assert_eq!(ret, 0, "connect on valid socket should return 0 (offline)");
}

#[test]
fn test_bind_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let addr = 0x30000u64;
    emu.maps
        .create_map("sockaddr", addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_word(addr, 2);
    emu.maps.write_word(addr + 2, 8080u16.to_be());
    emu.maps.write_dword(addr + 4, 0);

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::bind, &[sock, addr, 16]);
    assert_eq!(ret, 0, "bind on valid socket should return 0");
}

#[test]
fn test_listen_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::listen, &[sock, 5]);
    assert_eq!(ret, 0, "listen on valid socket should return 0");
}

#[test]
fn test_accept_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let addr = 0x30000u64;
    emu.maps
        .create_map("accept_buf", addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::accept, &[sock, addr, 16, 0]);
    assert_ne!(
        ret, !0u64,
        "accept on valid socket should not return INVALID_SOCKET"
    );
    assert!(ret > 0, "accept should return a new socket handle");
}

#[test]
fn test_setsockopt_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let optval_addr = 0x30000u64;
    emu.maps
        .create_map("optval", optval_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_dword(optval_addr, 1);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::setsockopt,
        &[sock, 0xFFFF, 0x0004, optval_addr, 4],
    );
    assert_eq!(ret, 0, "setsockopt on valid socket should return 0");
}

#[test]
fn test_getsockopt_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let optval_addr = 0x30000u64;
    emu.maps
        .create_map("optval", optval_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::getsockopt,
        &[sock, 0xFFFF, 0x0004, optval_addr, 4],
    );
    assert_eq!(ret, 0, "getsockopt on valid socket should return 0");
    let val = emu.maps.read_dword(optval_addr).unwrap();
    assert_eq!(val, 1, "getsockopt should write 1 to optval");
}

#[test]
fn test_socket_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let s1 = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    let s2 = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(s1, s2, "two sockets should have different handles");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[s1]);
    assert_eq!(ret, 0);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[s2]);
    assert_eq!(ret, 0);
}

// ─── ws2_32 32-bit ───

#[test]
fn test_wsa_startup_32() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::WsaStartup, &[0x0202, 0]);
    assert_eq!(ret, 0, "WSAStartup 32-bit should return 0");
}

#[test]
fn test_socket_create_32() {
    helpers::setup();
    let mut emu = emu32();
    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(sock, 0, "socket() 32-bit should return a non-zero handle");
}

#[test]
fn test_htons_32() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::htons, &[80]);
    assert_eq!(
        ret as u16,
        80u16.to_be(),
        "htons(80) 32-bit should byte-swap"
    );
}

#[test]
fn test_connect_valid_socket_32() {
    helpers::setup();
    let mut emu = emu32();

    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);

    let addr = 0x30000u32;
    emu.maps
        .create_map("sockaddr", addr as u64, 0x1000, Permission::READ_WRITE);
    emu.maps.write_word(addr as u64, 2);
    emu.maps.write_word(addr as u64 + 2, 80u16.to_be());
    emu.maps.write_dword(addr as u64 + 4, 0x0100007f);

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::connect, &[sock, addr, 16]);
    assert_eq!(ret, 0, "connect 32-bit on valid socket should return 0");
}

#[test]
fn test_closesocket_32() {
    helpers::setup();
    let mut emu = emu32();
    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);
    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket 32-bit should return 0");
}

// ─── ole32 64-bit ───

#[test]
fn test_co_initialize_ex_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoInitializeEx, &[0, 0]);
    assert_eq!(ret, 0, "CoInitializeEx should return S_OK (0)");
}

#[test]
fn test_co_create_instance_class_not_reg_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("com_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(buf + 0x100, 0xDEAD);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoCreateInstance,
        &[buf, 0, 0x17, buf + 0x10, buf + 0x100],
    );
    assert_eq!(
        ret, 0x80040154,
        "CoCreateInstance should return REGDB_E_CLASSNOTREG"
    );
    let ppv = emu.maps.read_qword(buf + 0x100).unwrap();
    assert_eq!(ppv, 0, "ppv should be set to NULL");
}

#[test]
fn test_co_task_mem_alloc_64() {
    helpers::setup();
    let mut emu = emu64();
    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc should return non-zero pointer");
    assert!(
        emu.maps.is_mapped(ptr),
        "CoTaskMemAlloc pointer should be mapped"
    );
}

#[test]
fn test_co_task_mem_free_64() {
    helpers::setup();
    let mut emu = emu64();
    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x100]);
    assert_ne!(ptr, 0);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);
    assert_eq!(ret, 0, "CoTaskMemFree should return 0");
}

#[test]
fn test_co_task_mem_free_null_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[0]);
    assert_eq!(ret, 0, "CoTaskMemFree(NULL) should not crash");
}

#[test]
fn test_co_uninitialize_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
    assert_eq!(ret, 0, "CoUninitialize should return 0");
}

#[test]
fn test_co_task_mem_alloc_free_cycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x100]);
    let p2 = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(p1, 0);
    assert_ne!(p2, 0);
    assert_ne!(p1, p2, "two allocations should differ");

    emu.maps.write_dword(p1, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(p1).unwrap(), 0xCAFEBABE);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[p1]);
    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[p2]);
}

// ─── oleaut32 64-bit ───

#[test]
fn test_sys_alloc_string_len_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 32]);
    assert_ne!(bstr, 0, "SysAllocStringLen should return non-zero BSTR");
    assert!(emu.maps.is_mapped(bstr), "BSTR pointer should be mapped");
}

#[test]
fn test_sys_free_string_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 16]);
    assert_ne!(bstr, 0);
    helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysFreeString, &[bstr]);
}

#[test]
fn test_variant_clear_64() {
    helpers::setup();
    let mut emu = emu64();

    let variant = 0x30000u64;
    emu.maps
        .create_map("variant", variant, 0x1000, Permission::READ_WRITE);
    // Write some non-zero data to simulate a VARIANT
    emu.maps.write_qword(variant, 0xDEADBEEF);
    emu.maps.write_qword(variant + 8, 0xCAFEBABE);

    let ret = helpers::call_winapi64(&mut emu, winapi64::oleaut32::VariantClear, &[variant]);
    assert_eq!(ret, 0, "VariantClear should return S_OK (0)");

    let vt = emu.maps.read_word(variant).unwrap();
    assert_eq!(vt, 0, "VariantClear should set vt to VT_EMPTY (0)");
}

// ─── oleaut32 new functions 64-bit ───

#[test]
fn test_sys_alloc_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("wide_buf", buf, 0x1000, Permission::READ_WRITE);
    // Write "Hi" as wide string (H=0x48, i=0x69, null=0x00)
    emu.maps.write_word(buf, 0x48);
    emu.maps.write_word(buf + 2, 0x69);
    emu.maps.write_word(buf + 4, 0x00);

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocString, &[buf]);
    assert_ne!(bstr, 0, "SysAllocString should return non-zero BSTR");

    // Read length prefix (4 bytes before BSTR pointer)
    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap();
    assert_eq!(
        len_bytes, 4,
        "BSTR length prefix should be 4 bytes (2 chars)"
    );
}

#[test]
fn test_sys_string_len_64() {
    helpers::setup();
    let mut emu = emu64();

    // First allocate a BSTR with SysAllocStringLen (known to work)
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 10]);
    assert_ne!(bstr, 0);

    // Now read the length back
    let len = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysStringLen, &[bstr]);
    assert_eq!(len, 10, "SysStringLen should return 10");
}

#[test]
fn test_sys_string_byte_len_64() {
    helpers::setup();
    let mut emu = emu64();

    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 10]);
    assert_ne!(bstr, 0);

    let byte_len = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysStringByteLen, &[bstr]);
    assert_eq!(
        byte_len, 20,
        "SysStringByteLen should return 20 (10 chars * 2)"
    );
}

#[test]
fn test_variant_init_64() {
    helpers::setup();
    let mut emu = emu64();

    let variant = 0x30000u64;
    emu.maps
        .create_map("variant", variant, 0x1000, Permission::READ_WRITE);
    // Fill with non-zero data
    for i in 0..16u64 {
        emu.maps.write_byte(variant + i, 0xFF);
    }

    helpers::call_winapi64(&mut emu, winapi64::oleaut32::VariantInit, &[variant]);

    // All 16 bytes should be zero
    for i in 0..16u64 {
        let b = emu.maps.read_byte(variant + i).unwrap();
        assert_eq!(b, 0, "VariantInit should zero byte at offset {}", i);
    }
}
