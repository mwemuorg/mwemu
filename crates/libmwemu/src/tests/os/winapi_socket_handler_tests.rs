use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;
use crate::*;

// ─── send/recv 64-bit: validates real buffer management ───

#[test]
fn send_valid_socket_returns_len_64() {
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
fn send_invalid_socket_returns_zero_64() {
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
fn recv_valid_socket_fills_buffer_64() {
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
fn recv_invalid_socket_returns_error_64() {
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
fn recv_4_byte_writes_size_64() {
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
fn send_loop_termination_64() {
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
fn recv_loop_termination_64() {
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
fn send_valid_socket_32() {
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
fn recv_valid_socket_32() {
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
fn inet_addr_parses_ip_64() {
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
fn inet_addr_192_168_1_1_64() {
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
fn getaddrinfo_creates_result_64() {
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
fn handler_create_returns_unique_ids() {
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
fn handler_exist_tracks_lifecycle() {
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
fn handler_uri_lookup() {
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
fn handler_find_by_uri() {
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
fn handler_seek_and_tell() {
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
fn handler_put_and_read_bytes() {
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
fn socket_create_returns_unique_ids() {
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
fn socket_exist_tracks_lifecycle() {
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
fn crypto_pipeline_md5_through_winapi_64() {
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
fn crypto_genkey_encrypt_decrypt_64() {
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
        encrypted.as_ref(),
        plaintext,
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
        decrypted.as_ref(),
        plaintext,
        "decrypted data should match original plaintext"
    );
}

#[test]
fn crypto_gen_random_64() {
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
fn internet_open_a_creates_handle_64() {
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
fn internet_open_w_creates_handle_64() {
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
fn internet_connect_a_creates_handle_64() {
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

// ─── advance_tick test ───

#[test]
fn advance_tick_increments_time() {
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

// ─── IP Helper queries through the x86/x64 DLL gateways ───

const EMPTY_MIB_TABLE_SIZE: u32 = 4;
const IP_HELPER_CANARY: u8 = 0xa5;

fn call_ip_helper(emu: &mut emu::Emu, name: &str, args: &[u64]) -> u64 {
    helpers::call_winapi_export(emu, "iphlpapi.dll", name, args)
}

#[test]
fn ip_helper_tables_support_size_probe_and_empty_result() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.cfg.skip_unimplemented = false;
        emu.cfg.stop_on_unimplemented = true;
        emu.set_last_error(ERROR_ACCESS_DENIED);
        let size = data + 1;
        let table = data + 0x100;
        for name in ["GetIfTable", "GetIpAddrTable", "GetTcpTable", "GetUdpTable"] {
            assert!(emu.maps.write_bytes(table - 1, &[IP_HELPER_CANARY; 6]));
            for (buffer, capacity) in [(0, 0), (table, EMPTY_MIB_TABLE_SIZE - 1)] {
                assert!(emu.maps.write_dword(size, capacity));
                assert_eq!(
                    call_ip_helper(&mut emu, name, &[buffer, size, 1]),
                    ERROR_INSUFFICIENT_BUFFER,
                    "{name}"
                );
                assert_eq!(emu.maps.read_dword(size), Some(EMPTY_MIB_TABLE_SIZE));
                assert_eq!(
                    emu.maps.read_bytes(table - 1, 6).as_ref(),
                    &[IP_HELPER_CANARY; 6]
                );
            }
            assert_eq!(
                call_ip_helper(&mut emu, name, &[table, size, 0]),
                ERROR_SUCCESS,
                "{name}"
            );
            assert_eq!(emu.maps.read_dword(table), Some(0));
            assert_eq!(emu.maps.read_byte(table - 1), Some(IP_HELPER_CANARY));
            assert_eq!(
                emu.maps.read_byte(table + EMPTY_MIB_TABLE_SIZE as u64),
                Some(IP_HELPER_CANARY)
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            assert!(!emu.process_terminated);
        }
    }
}

#[test]
fn ip_helper_network_parameters_use_the_guest_fixed_info_layout() {
    helpers::setup();
    // FIXED_INFO: x86 has 4-byte pointers; x64 has 8-byte pointers and padding.
    for (is64, required, node_type) in [(false, 584u32, 308u64), (true, 600, 320)] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let size = data + 1;
        let output = data + 0x100;
        emu.cfg.host_name = "LAB-HOST".into();
        assert!(
            emu.maps
                .write_bytes(output - 1, &vec![IP_HELPER_CANARY; required as usize + 2])
        );
        for (buffer, capacity) in [(0, 0), (output, required - 1)] {
            assert!(emu.maps.write_dword(size, capacity));
            assert_eq!(
                call_ip_helper(&mut emu, "GetNetworkParams", &[buffer, size]),
                ERROR_BUFFER_OVERFLOW
            );
            assert_eq!(emu.maps.read_dword(size), Some(required));
            assert_eq!(emu.maps.read_byte(output), Some(IP_HELPER_CANARY));
        }
        assert_eq!(
            call_ip_helper(&mut emu, "GetNetworkParams", &[output, size]),
            ERROR_SUCCESS
        );
        let mut expected = vec![0; required as usize];
        expected[..8].copy_from_slice(b"LAB-HOST");
        expected[node_type as usize..node_type as usize + 4]
            .copy_from_slice(&BROADCAST_NODETYPE.to_le_bytes());
        assert_eq!(
            emu.maps.read_bytes(output, required as usize).as_ref(),
            expected
        );
        assert_eq!(emu.maps.read_byte(output - 1), Some(IP_HELPER_CANARY));
        assert_eq!(
            emu.maps.read_byte(output + required as u64),
            Some(IP_HELPER_CANARY)
        );

        emu.cfg.host_name = "A".repeat(256);
        assert_eq!(
            call_ip_helper(&mut emu, "GetNetworkParams", &[output, size]),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_bytes(output, 131).as_ref(), &[b'A'; 131]);
        assert_eq!(
            emu.maps.read_word(output + 131),
            Some(0),
            "terminated hostname must not overwrite the domain"
        );
    }
}

#[test]
fn ip_helper_rejects_invalid_buffers_without_partial_writes() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let size = data + 1;
        let output = data + 0x100;
        let unmapped = data + 0x10000;
        let read_only = data + 0x20000;
        let short_output = data + 0x30000;
        emu.maps
            .create_map("ip_helper_read_only", read_only, 0x1000, Permission::READ)
            .unwrap();
        emu.maps
            .create_map("ip_helper_short", short_output, 3, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_bytes(short_output, &[IP_HELPER_CANARY; 3]));
        assert!(emu.maps.write_bytes(output, &[IP_HELPER_CANARY; 600]));
        for (name, required, arguments) in [
            ("GetIfTable", 4, 3),
            ("GetIpAddrTable", 4, 3),
            ("GetTcpTable", 4, 3),
            ("GetUdpTable", 4, 3),
            ("GetNetworkParams", if is64 { 600 } else { 584 }, 2),
        ] {
            for bad_size in [0, unmapped, read_only, data + 0xfff] {
                let args = [output, bad_size, 0];
                assert_eq!(
                    call_ip_helper(&mut emu, name, &args[..arguments]),
                    ERROR_INVALID_PARAMETER,
                    "{name}: size pointer"
                );
                assert_eq!(
                    emu.maps.read_bytes(output, 600).as_ref(),
                    &[IP_HELPER_CANARY; 600]
                );
            }
            for bad_output in [unmapped, read_only, short_output] {
                assert!(emu.maps.write_dword(size, required));
                let args = [bad_output, size, 0];
                assert_eq!(
                    call_ip_helper(&mut emu, name, &args[..arguments]),
                    ERROR_INVALID_PARAMETER,
                    "{name}: output buffer"
                );
                assert_eq!(emu.maps.read_dword(size), Some(required));
                assert_eq!(
                    emu.maps.read_bytes(short_output, 3).as_ref(),
                    &[IP_HELPER_CANARY; 3]
                );
            }
        }
    }
}

#[test]
fn ip_helper_adapter_queries_report_no_data_and_validate_parameters() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let size = data + 1;
        let output = data + 0x100;
        let unmapped = data + 0x10000;
        let read_only = data + 0x20000;
        emu.maps
            .create_map("adapter_read_only", read_only, 0x1000, Permission::READ)
            .unwrap();
        assert!(emu.maps.write_dword(size, 512));
        assert!(emu.maps.write_bytes(output, &[IP_HELPER_CANARY; 512]));
        for buffer in [0, output] {
            assert_eq!(
                call_ip_helper(&mut emu, "GetAdaptersInfo", &[buffer, size]),
                ERROR_NO_DATA
            );
            for family in [AF_UNSPEC, AF_INET, AF_INET6] {
                assert_eq!(
                    call_ip_helper(
                        &mut emu,
                        "GetAdaptersAddresses",
                        &[family as u64, 0, 0, buffer, size]
                    ),
                    ERROR_NO_DATA
                );
            }
        }
        for bad_size in [0, unmapped, read_only, data + 0xfff] {
            assert_eq!(
                call_ip_helper(&mut emu, "GetAdaptersInfo", &[output, bad_size]),
                ERROR_INVALID_PARAMETER
            );
            assert_eq!(
                call_ip_helper(
                    &mut emu,
                    "GetAdaptersAddresses",
                    &[AF_INET as u64, 0, 0, output, bad_size]
                ),
                ERROR_INVALID_PARAMETER
            );
        }
        for (family, reserved) in [(u32::MAX, 0), (AF_INET, output)] {
            assert_eq!(
                call_ip_helper(
                    &mut emu,
                    "GetAdaptersAddresses",
                    &[family as u64, 0, reserved, output, size]
                ),
                ERROR_INVALID_PARAMETER
            );
        }
        assert_eq!(emu.maps.read_dword(size), Some(512));
        assert_eq!(
            emu.maps.read_bytes(output, 512).as_ref(),
            &[IP_HELPER_CANARY; 512]
        );
    }
}

#[test]
fn ip_helper_unknown_exports_keep_the_existing_fallback() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        emu.cfg.skip_unimplemented = false;
        call_ip_helper(&mut emu, "UnsupportedIpHelperQuery", &[]);
        assert!(emu.process_terminated);
    }
}
