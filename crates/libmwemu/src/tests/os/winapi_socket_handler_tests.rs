use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::*;

// ─── send/recv 64-bit: validates real buffer management ───

#[test]
fn test_send_valid_socket_returns_len_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(sock, 0);

    let buf = 0x30000u64;
    emu.maps
        .create_map("sendbuf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_bytes(buf, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::send, &[sock, buf, 5, 0]);
    assert_eq!(ret, 5, "send should return the number of bytes sent");
}

#[test]
fn test_send_invalid_socket_returns_zero_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("sendbuf", buf, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::send,
        &[0xDEAD, buf, 5, 0], // invalid socket
    );
    assert_eq!(ret, 0, "send on invalid socket should return 0");
}

#[test]
fn test_recv_valid_socket_fills_buffer_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let buf = 0x30000u64;
    emu.maps
        .create_map("recvbuf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.memset(buf, 0x00, 32);

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::recv, &[sock, buf, 16, 0]);
    assert_eq!(ret, 16, "recv should return the requested length");

    let byte = emu.maps.read_byte(buf).unwrap();
    assert_ne!(byte, 0x00, "recv should fill the buffer with data (0x90)");
}

#[test]
fn test_recv_invalid_socket_returns_error_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("recvbuf", buf, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::recv,
        &[0xBAD, buf, 16, 0], // invalid socket
    );
    assert_eq!(ret, 1, "recv on invalid socket should return error (1)");
}

#[test]
fn test_recv_4_byte_writes_size_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let buf = 0x30000u64;
    emu.maps
        .create_map("recvbuf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_dword(buf, 0);

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::recv, &[sock, buf, 4, 0]);
    assert_eq!(ret, 4, "recv of 4 bytes should return 4");

    let val = emu.maps.read_dword(buf).unwrap();
    assert_eq!(val, 0x0100, "recv of 4 bytes should write a size value");
}

#[test]
fn test_send_loop_termination_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let buf = 0x30000u64;
    emu.maps
        .create_map("sendbuf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_bytes(buf, &[0x41; 32]);

    let mut returns = Vec::new();
    for _ in 0..5 {
        let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::send, &[sock, buf, 32, 0]);
        returns.push(ret);
    }
    assert!(
        returns.contains(&0),
        "after >3 sends, send should return 0 to terminate loops: {:?}",
        returns
    );
}

#[test]
fn test_recv_loop_termination_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(&mut emu, winapi64::ws2_32::socket, &[2, 1, 6]);

    let buf = 0x30000u64;
    emu.maps
        .create_map("recvbuf", buf, 0x1000, Permission::READ_WRITE);

    let mut returns = Vec::new();
    for _ in 0..5 {
        let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::recv, &[sock, buf, 32, 0]);
        returns.push(ret);
    }
    assert!(
        returns.contains(&0),
        "after >3 recvs, recv should return 0 to terminate loops: {:?}",
        returns
    );
}

// ─── send/recv 32-bit ───

#[test]
fn test_send_valid_socket_32() {
    helpers::setup();
    let mut emu = emu32();

    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);
    assert_ne!(sock, 0);

    let buf = 0x30000u32;
    emu.maps
        .create_map("sendbuf", buf as u64, 0x1000, Permission::READ_WRITE);
    emu.maps
        .write_bytes(buf as u64, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]);

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::send, &[sock, buf, 5, 0]);
    assert_eq!(ret, 5, "send 32-bit should return bytes sent");
}

#[test]
fn test_recv_valid_socket_32() {
    helpers::setup();
    let mut emu = emu32();

    let sock = helpers::call_winapi32(&mut emu, winapi32::ws2_32::socket, &[2, 1, 6]);

    let buf = 0x30000u32;
    emu.maps
        .create_map("recvbuf", buf as u64, 0x1000, Permission::READ_WRITE);
    emu.maps.memset(buf as u64, 0x00, 32);

    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::recv, &[sock, buf, 16, 0]);
    assert_eq!(ret, 16, "recv 32-bit should return requested length");
}

// ─── inet_addr 64-bit: real IP parsing ───

#[test]
fn test_inet_addr_parses_ip_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("ipstr", str_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(str_addr, "127.0.0.1");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::inet_addr, &[str_addr]);
    assert_eq!(
        ret, 0x0100007f,
        "inet_addr(\"127.0.0.1\") should return 0x0100007f"
    );
}

#[test]
fn test_inet_addr_192_168_1_1_64() {
    helpers::setup();
    let mut emu = emu64();

    let str_addr = 0x30000u64;
    emu.maps
        .create_map("ipstr", str_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(str_addr, "192.168.1.1");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::inet_addr, &[str_addr]);
    let expected = u32::from_ne_bytes([192, 168, 1, 1]) as u64;
    assert_eq!(
        ret, expected,
        "inet_addr(\"192.168.1.1\") should parse correctly"
    );
}

// ─── getaddrinfo 64-bit: handler-based DNS ───

#[test]
fn test_getaddrinfo_creates_result_64() {
    helpers::setup();
    let mut emu = emu64();

    let hostname = 0x30000u64;
    let result_ptr = 0x31000u64;
    emu.maps
        .create_map("addrbuf", hostname, 0x2000, Permission::READ_WRITE);
    emu.maps.write_string(hostname, "example.com");
    emu.maps.write_qword(result_ptr, 0);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::getaddrinfo,
        &[hostname, 0, 0, result_ptr],
    );
    assert_eq!(ret, 0, "getaddrinfo should return 0 (success)");

    let result = emu.maps.read_qword(result_ptr).unwrap_or(0);
    assert_ne!(
        result, 0,
        "getaddrinfo should write a non-null result pointer"
    );
}

// ─── handler lifecycle: create, exist, close ───

#[test]
fn test_handler_create_returns_unique_ids() {
    use crate::winapi::helper;

    let h1 = helper::handler_create("file://test1");
    let h2 = helper::handler_create("file://test2");
    let h3 = helper::handler_create("cryptctx://prov");
    assert_ne!(h1, h2, "handler ids must be unique");
    assert_ne!(h2, h3, "handler ids must be unique");
    assert_ne!(h1, 0, "handler id must be non-zero");

    helper::handler_close(h1);
    helper::handler_close(h2);
    helper::handler_close(h3);
}

#[test]
fn test_handler_exist_tracks_lifecycle() {
    use crate::winapi::helper;

    let h = helper::handler_create("test://lifecycle");
    assert!(
        helper::handler_exist(h),
        "handler should exist after create"
    );

    assert!(helper::handler_close(h), "close should return true");
    assert!(
        !helper::handler_exist(h),
        "handler should not exist after close"
    );

    assert!(
        !helper::handler_close(h),
        "closing an already-closed handler should return false"
    );
}

#[test]
fn test_handler_uri_lookup() {
    use crate::winapi::helper;

    let h = helper::handler_create("file://C:\\Windows\\System32\\kernel32.dll");
    let uri = helper::handler_get_uri(h);
    assert_eq!(uri, "file://C:\\Windows\\System32\\kernel32.dll");

    let not_found = helper::handler_get_uri(0xFFFF);
    assert_eq!(
        not_found, "",
        "non-existent handler should return empty URI"
    );

    helper::handler_close(h);
}

#[test]
fn test_handler_find_by_uri() {
    use crate::winapi::helper;

    let unique_uri = format!("inet://test-find-{}", std::process::id());
    let h = helper::handler_create(&unique_uri);
    let found = helper::handler_find_by_uri(&unique_uri);
    assert_eq!(found, Some(h), "should find handler by its URI");

    let not_found = helper::handler_find_by_uri("inet://surely-never-created-9999");
    assert_eq!(not_found, None, "should not find non-existent URI");

    helper::handler_close(h);
}

#[test]
fn test_handler_seek_and_tell() {
    use crate::winapi::helper;

    let h = helper::handler_create("file://test_seek");
    helper::handler_put_bytes(h, &[0u8; 100]);

    let pos = helper::handler_seek(h, 50, 0); // SEEK_SET
    assert_eq!(pos, 50);
    assert_eq!(helper::handler_tell(h), 50);

    let pos = helper::handler_seek(h, 10, 1); // SEEK_CUR
    assert_eq!(pos, 60);

    let pos = helper::handler_seek(h, -5, 2); // SEEK_END
    assert_eq!(pos, 95);

    helper::handler_close(h);
}

#[test]
fn test_handler_put_and_read_bytes() {
    use crate::winapi::helper;

    let h = helper::handler_create("file://data_test");
    let data = b"Hello, World!";
    helper::handler_put_bytes(h, data);

    assert!(!helper::handler_eof(h), "not at EOF initially");
    helper::handler_set_eof(h);
    assert!(helper::handler_eof(h), "should be at EOF after set_eof");

    helper::handler_seek(h, 0, 0);
    assert!(!helper::handler_eof(h), "seek should clear EOF");

    helper::handler_close(h);
}

// ─── socket lifecycle: create, exist, close ───

#[test]
fn test_socket_create_returns_unique_ids() {
    use crate::winapi::helper;

    let s1 = helper::socket_create();
    let s2 = helper::socket_create();
    assert_ne!(s1, s2, "socket ids must be unique");
    assert!(
        s1 >= 3,
        "sockets should start at 3 (above stdin/stdout/stderr)"
    );

    helper::socket_close(s1);
    helper::socket_close(s2);
}

#[test]
fn test_socket_exist_tracks_lifecycle() {
    use crate::winapi::helper;

    let s = helper::socket_create();
    assert!(helper::socket_exist(s), "socket should exist after create");

    assert!(helper::socket_close(s), "close should return true");
    assert!(
        !helper::socket_exist(s),
        "socket should not exist after close"
    );

    assert!(
        !helper::socket_close(s),
        "closing an already-closed socket should return false"
    );
}

// ─── full crypto pipeline through advapi32 64-bit ───

#[test]
fn test_crypto_pipeline_md5_through_winapi_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("crypto_buf", buf, 0x2000, Permission::READ_WRITE);

    // CryptAcquireContextA: out_handle at buf, null container, null provider, type=1, flags=0xF0000000
    emu.maps.write_qword(buf, 0);
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[buf, 0, 0, 1, 0xF0000000],
    );
    assert_eq!(ret, 1, "CryptAcquireContextA should return TRUE");

    let hprov = emu.maps.read_qword(buf).unwrap_or(0);
    assert_ne!(
        hprov, 0,
        "CryptAcquireContextA should write a non-zero handle"
    );

    // CryptCreateHash: hprov, CALG_MD5=0x8003, 0, 0, hash_ptr at buf+0x100
    emu.maps.write_qword(buf + 0x100, 0);
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptCreateHash,
        &[hprov, 0x8003, 0, 0, buf + 0x100],
    );
    assert_eq!(ret, 1, "CryptCreateHash should return TRUE");

    let hhash = emu.maps.read_qword(buf + 0x100).unwrap_or(0);
    assert_ne!(
        hhash, 0,
        "CryptCreateHash should write a non-zero hash handle"
    );

    // CryptHashData: hash "password"
    let data_addr = buf + 0x200;
    emu.maps.write_bytes(data_addr, b"password");
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptHashData,
        &[hhash, data_addr, 8, 0],
    );
    assert_eq!(ret, 1, "CryptHashData should return TRUE");

    // CryptReleaseContext
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptReleaseContext,
        &[hprov, 0],
    );
    assert_eq!(ret, 1, "CryptReleaseContext should return TRUE");
}

#[test]
fn test_crypto_genkey_encrypt_decrypt_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("crypto_buf", buf, 0x2000, Permission::READ_WRITE);

    // CryptAcquireContextA
    emu.maps.write_qword(buf, 0);
    helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[buf, 0, 0, 1, 0xF0000000],
    );
    let hprov = emu.maps.read_qword(buf).unwrap_or(0);

    // CryptGenKey: CALG_RC4=0x6801, 40-bit key (0x00280000)
    let hkey_ptr = buf + 0x100;
    emu.maps.write_qword(hkey_ptr, 0);
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptGenKey,
        &[hprov, 0x6801, 0x00280000, hkey_ptr],
    );
    assert_eq!(ret, 1, "CryptGenKey should return TRUE");

    let hkey = emu.maps.read_qword(hkey_ptr).unwrap_or(0);
    assert_ne!(hkey, 0, "CryptGenKey should write a non-zero key handle");

    // Write plaintext
    let plaintext = b"SecretMessage!!!";
    let data_addr = buf + 0x200;
    let data_len_ptr = buf + 0x300;
    emu.maps.write_bytes(data_addr, plaintext);
    emu.maps.write_dword(data_len_ptr, plaintext.len() as u32);

    // CryptEncrypt
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptEncrypt,
        &[hkey, 0, 1, 0, data_addr, data_len_ptr, 256],
    );
    assert_eq!(ret, 1, "CryptEncrypt should return TRUE");

    let encrypted = emu.maps.read_bytes(data_addr, plaintext.len());
    assert_ne!(
        encrypted, plaintext,
        "encrypted data should differ from plaintext"
    );

    // CryptDecrypt
    emu.maps.write_dword(data_len_ptr, plaintext.len() as u32);
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptDecrypt,
        &[hkey, 0, 1, 0, data_addr, data_len_ptr],
    );
    assert_eq!(ret, 1, "CryptDecrypt should return TRUE");

    let decrypted = emu.maps.read_bytes(data_addr, plaintext.len());
    assert_eq!(
        decrypted, plaintext,
        "decrypted data should match original plaintext"
    );
}

#[test]
fn test_crypto_gen_random_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("crypto_buf", buf, 0x2000, Permission::READ_WRITE);

    // CryptAcquireContextA
    emu.maps.write_qword(buf, 0);
    helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[buf, 0, 0, 1, 0xF0000000],
    );
    let hprov = emu.maps.read_qword(buf).unwrap_or(0);

    // Zero the output buffer
    let rand_buf = buf + 0x100;
    for i in 0..32u64 {
        emu.maps.write_byte(rand_buf + i, 0);
    }

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptGenRandom,
        &[hprov, 32, rand_buf],
    );
    assert_eq!(ret, 1, "CryptGenRandom should return TRUE");

    let bytes = emu.maps.read_bytes(rand_buf, 32);
    let all_zero = bytes.iter().all(|&b| b == 0);
    assert!(
        !all_zero,
        "CryptGenRandom should fill buffer with non-zero data"
    );
}

// ─── InternetOpen/InternetConnect handler tests 64-bit ───

#[test]
fn test_internet_open_a_creates_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("inet_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(buf, "Mozilla/5.0");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::wininet::InternetOpenA,
        &[buf, 0, 0, 0, 0],
    );
    assert_ne!(ret, 0, "InternetOpenA should return a non-zero handle");
}

#[test]
fn test_internet_open_w_creates_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("inet_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(buf, "Mozilla/5.0");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::wininet::InternetOpenW,
        &[buf, 0, 0, 0, 0],
    );
    assert_ne!(ret, 0, "InternetOpenW should return a non-zero handle");
}

#[test]
fn test_internet_connect_a_creates_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("inet_buf", buf, 0x2000, Permission::READ_WRITE);

    // First open an internet session
    emu.maps.write_string(buf, "Agent");
    let hinet = helpers::call_winapi64(
        &mut emu,
        winapi64::wininet::InternetOpenA,
        &[buf, 0, 0, 0, 0],
    );

    // Then connect
    let host = buf + 0x100;
    emu.maps.write_string(host, "example.com");
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::wininet::InternetConnectA,
        &[hinet, host, 80, 0, 0, 3, 0, 0], // INTERNET_SERVICE_HTTP=3
    );
    assert_ne!(ret, 0, "InternetConnectA should return a non-zero handle");
    assert_ne!(
        ret, hinet,
        "connect handle should differ from session handle"
    );
}

// ─── CreateFileA/W 64-bit: handler-based file handles ───
// CreateFile needs filesystem initialization to resolve paths; without it,
// it correctly returns INVALID_HANDLE_VALUE (-1). Test that it reads the
// filename and returns -1 rather than crashing (no filesystem = no file).

#[test]
fn test_create_file_a_no_filesystem_returns_invalid_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("file_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(buf, "C:\\test.txt");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::CreateFileA,
        &[buf, 0x80000000, 0, 0, 3, 0, 0],
    );
    assert_eq!(
        ret, 0xFFFFFFFFFFFFFFFF,
        "CreateFileA without filesystem should return INVALID_HANDLE_VALUE"
    );
}

#[test]
fn test_create_file_w_no_filesystem_returns_invalid_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("file_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(buf, "C:\\test.txt");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::CreateFileW,
        &[buf, 0x80000000, 0, 0, 3, 0, 0],
    );
    assert_eq!(
        ret, 0xFFFFFFFFFFFFFFFF,
        "CreateFileW without filesystem should return INVALID_HANDLE_VALUE"
    );
}

// ─── advance_tick test ───

#[test]
fn test_advance_tick_increments_time() {
    use crate::winapi::helper;

    helpers::setup();
    let mut emu = emu64();
    let tick_before = emu.tick;
    helper::advance_tick(&mut emu, 1000);
    assert!(
        emu.tick > tick_before,
        "advance_tick should increment emu.tick"
    );
}
