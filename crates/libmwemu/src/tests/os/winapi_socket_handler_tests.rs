use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers;
use crate::winapi::common::{heap, memory, strings};
use crate::winapi::policy::{ApiDisposition, ApiMode, ApiStopReason};
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;
use crate::*;

#[test]
fn legacy_ipv4_conversion_supports_classful_notation_and_thread_storage() {
    for is64 in [false, true] {
        let (mut emu, source) = helpers::prepare_winapi(is64);
        for (text, expected) in [
            ("127.1", [127, 0, 0, 1]),
            ("0x7f000001", [127, 0, 0, 1]),
            ("0177.0.1", [127, 0, 0, 1]),
            ("1.2.65535", [1, 2, 255, 255]),
            ("1.2.3.4 tail", [1, 2, 3, 4]),
            ("08.1", [255; 4]),
            ("1.2.3.400", [255; 4]),
        ] {
            emu.maps.write_string(source, text);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", "inet_addr", &[source]),
                u32::from_le_bytes(expected) as u64,
                "{text}"
            );
        }
        let format = |emu: &mut emu::Emu, ip| {
            helpers::call_winapi_export(
                emu,
                "ws2_32.dll",
                "inet_ntoa",
                &[u32::from_le_bytes(ip).into()],
            )
        };
        let buffer = format(&mut emu, [127, 0, 0, 1]);
        assert_ne!(buffer, 0);
        assert_eq!(emu.maps.read_string(buffer), "127.0.0.1");
        assert_eq!(format(&mut emu, [192, 168, 1, 1]), buffer);
        assert_eq!(emu.maps.read_string(buffer), "192.168.1.1");
    }
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
    emu.maps.write_string(hostname, "localhost");
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
fn ip_helper_unknown_exports_report_unsupported_calls() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        emu.cfg.skip_unimplemented = false;
        helpers::invoke_winapi_export(&mut emu, "iphlpapi.dll", "UnsupportedIpHelperQuery");
        assert!(emu.api_policy.stopped());
        assert!(!emu.process_terminated);
    }
}

#[test]
fn winsock_exports_preserve_stack_and_use_architecture_layouts() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let layout_size = if is64 { 408 } else { 400 };
        let description = if is64 { 16 } else { 4 };
        let status = description + WSADESCRIPTION_LEN as u64 + 1;
        emu.maps.memset(data, 0xa5, 512);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "ws2_32.dll", "WSAStartup", &[0x0202, data]),
            0
        );
        assert_eq!(emu.maps.read_word(data), Some(0x0202));
        assert_eq!(emu.maps.read_word(data + 2), Some(0x0202));
        assert_eq!(emu.maps.read_string(data + description), "WinSock 2.0");
        assert_eq!(emu.maps.read_string(status + data), "Running");
        assert_eq!(emu.maps.read_byte(data + layout_size), Some(0xa5));
        for name in ["WSASocketA", "WSASocketW"] {
            let socket = helpers::call_winapi_export(
                &mut emu,
                "ws2_32.dll",
                name,
                &[
                    AF_INET as u64,
                    SOCK_STREAM as u64,
                    IPPROTO_TCP as u64,
                    0,
                    0,
                    0,
                ],
            );
            assert!(emu.sockets.contains(socket));
            let address = data + 512;
            let length = data + 544;
            write_socket_address(&mut emu, address, AF_INET, 0);
            assert_eq!(socket_call(&mut emu, "bind", &[socket, address, 16]), 0);
            assert!(emu.maps.write_dword(length, 16));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "ws2_32.dll",
                    "getsockname",
                    &[socket, address, length]
                ),
                0
            );
            assert_eq!(emu.maps.read_word(address), Some(AF_INET as u16));
            assert_eq!(emu.maps.read_dword(length), Some(16));
            helpers::call_winapi_export(&mut emu, "ws2_32.dll", "WSASetLastError", &[WSAENOTSOCK]);
            for _ in 0..2 {
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, "ws2_32.dll", "WSAGetLastError", &[]),
                    WSAENOTSOCK
                );
            }
            assert_eq!(socket_call(&mut emu, "closesocket", &[socket]), 0);
        }
    }
}

#[test]
fn host_name_pairs_count_characters_and_reject_short_buffers() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.cfg.host_name = "TEST-PC".into();
        for (name, wide) in [("gethostname", false), ("GetHostNameW", true)] {
            let expected = strings::encode_text("TEST-PC\0", wide);
            assert!(
                emu.maps
                    .write_bytes(data, &vec![u8::MAX; expected.len() + 1])
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[data, 7]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEFAULT);
            assert_eq!(emu.maps.read_byte(data), Some(u8::MAX));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[data, 8]),
                0
            );
            assert_eq!(
                emu.maps.read_bytes(data, expected.len()).as_ref(),
                expected.as_slice()
            );
            assert_eq!(
                emu.maps.read_byte(data + expected.len() as u64),
                Some(u8::MAX)
            );
        }
    }
}

#[test]
fn name_info_pairs_write_host_and_service_using_all_seven_arguments() {
    const ADDRESS_SIZE: u64 = 16;
    const PORT_OFFSET: u64 = 2;
    const IPV4_OFFSET: u64 = 4;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let host = data + 0x100;
        let service = data + 0x200;
        assert!(emu.maps.write_word(data, AF_INET as u16));
        assert!(emu.maps.write_word(data + PORT_OFFSET, 443u16.to_be()));
        assert!(emu.maps.write_bytes(data + IPV4_OFFSET, &[127, 0, 0, 1]));
        for (name, wide) in [("getnameinfo", false), ("GetNameInfoW", true)] {
            let expected = strings::encode_text("localhost\0", wide);
            assert!(
                emu.maps
                    .write_bytes(host, &vec![u8::MAX; expected.len() + 1])
            );
            assert!(emu.maps.write_byte(service, u8::MAX));
            let mut args = [data, ADDRESS_SIZE, host, 10, service, 4, 0];
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                WSAEFAULT
            );
            assert_eq!(emu.maps.read_byte(host), Some(u8::MAX));
            assert_eq!(emu.maps.read_byte(service), Some(u8::MAX));
            args[5] = 6;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                0
            );
            assert_eq!(strings::read_text(&emu, host, wide), "localhost");
            assert_eq!(strings::read_text(&emu, service, wide), "https");
            assert_eq!(
                emu.maps.read_byte(host + expected.len() as u64),
                Some(u8::MAX)
            );
            args[6] = (NI_NUMERICHOST | NI_NUMERICSERV) as u64;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                0
            );
            assert_eq!(strings::read_text(&emu, host, wide), "127.0.0.1");
            assert_eq!(strings::read_text(&emu, service, wide), "443");
            args[2] = 0;
            args[4] = 0;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                WSAHOST_NOT_FOUND
            );
            args[2] = host;
            args[6] = NI_NAMEREQD as u64;
            assert!(emu.maps.write_bytes(data + IPV4_OFFSET, &[127, 0, 0, 2]));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                WSAHOST_NOT_FOUND
            );
            args[6] = 0;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &args),
                0
            );
            assert_eq!(strings::read_text(&emu, host, wide), "127.0.0.2");
            assert!(emu.maps.write_bytes(data + IPV4_OFFSET, &[127, 0, 0, 1]));
        }
    }
}

#[test]
fn address_info_pairs_use_native_layout_and_release_owned_results() {
    const ADDRESS_SIZE: u64 = 16;
    const SCALARS_SIZE: u64 = 4 * size_of::<u32>() as u64;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let output = data + 0x800;
        let service = data + 0x900;
        let pointer = memory::pointer_size(&emu);
        for (name, free, wide) in [
            ("getaddrinfo", "freeaddrinfo", false),
            ("GetAddrInfoW", "FreeAddrInfoW", true),
        ] {
            let node = if wide {
                "хост".repeat(40)
            } else {
                "host".repeat(40)
            };
            emu.cfg.host_name = node.clone();
            let canonical = node.clone();
            let hints = data + 0x700;
            const AI_CANONNAME: u32 = 2;
            assert!(emu.maps.write_dword(hints, AI_CANONNAME));
            emu.maps.write_dword(hints + 8, SOCK_DGRAM);
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text(&format!("{node}\0"), wide))
            );
            assert!(
                emu.maps
                    .write_bytes(service, &strings::encode_text("https\0", wide))
            );
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "ws2_32.dll",
                    name,
                    &[data, service, hints, output]
                ),
                0
            );
            let result = memory::read_pointer(&emu, output).unwrap();
            assert_ne!(result, 0);
            assert_eq!(emu.maps.read_dword(result + 8), Some(SOCK_DGRAM));
            assert_eq!(emu.maps.read_dword(result + 12), Some(IPPROTO_UDP));
            assert_eq!(
                emu.maps.read_dword(result + size_of::<u32>() as u64),
                Some(AF_INET)
            );
            assert_eq!(
                memory::read_pointer(&emu, result + SCALARS_SIZE),
                Some(ADDRESS_SIZE)
            );
            let canonical_pointer =
                memory::read_pointer(&emu, result + SCALARS_SIZE + pointer).unwrap();
            let address = memory::read_pointer(&emu, result + SCALARS_SIZE + 2 * pointer).unwrap();
            assert_eq!(
                memory::read_pointer(&emu, result + SCALARS_SIZE + 3 * pointer),
                Some(0)
            );
            assert_eq!(emu.maps.read_word(address), Some(AF_INET as u16));
            assert_eq!(
                emu.maps.read_word(address + size_of::<u16>() as u64),
                Some(443u16.to_be())
            );
            assert_eq!(strings::read_text(&emu, canonical_pointer, wide), canonical);
            let bytes = strings::encode_text(&format!("{canonical}\0"), wide).len() as u64;
            assert_eq!(
                heap::owned_size(&emu, 0, result),
                Some((SCALARS_SIZE + 4 * pointer + ADDRESS_SIZE + bytes) as usize)
            );
            helpers::call_winapi_export(&mut emu, "ws2_32.dll", free, &[result]);
            assert_eq!(heap::owned_size(&emu, 0, result), None);
            helpers::call_winapi_export(&mut emu, "ws2_32.dll", free, &[0]);
            emu.maps
                .write_bytes(data, &strings::encode_text("192.0.2.5\0", wide));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[data, 0, 0, output]),
                0
            );
            let result = memory::read_pointer(&emu, output).unwrap();
            assert_eq!(
                memory::read_pointer(&emu, result + SCALARS_SIZE + pointer),
                Some(0)
            );
            let address = memory::read_pointer(&emu, result + SCALARS_SIZE + 2 * pointer).unwrap();
            assert_eq!(
                emu.maps.read_dword(address + 4),
                Some(u32::from_le_bytes([192, 0, 2, 5]))
            );
            helpers::call_winapi_export(&mut emu, "ws2_32.dll", free, &[result]);
            emu.maps
                .write_bytes(data, &strings::encode_text("unresolved.invalid\0", wide));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[data, 0, 0, output]),
                WSAHOST_NOT_FOUND
            );
            assert_eq!(memory::read_pointer(&emu, output), Some(0));
        }
    }
}

#[test]
fn address_info_pairs_clear_result_on_unsupported_family() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let output = data + 0x100;
        assert!(
            emu.maps
                .write_dword(data + size_of::<u32>() as u64, AF_INET6)
        );
        for name in ["getaddrinfo", "GetAddrInfoW"] {
            assert!(memory::write_pointer(&mut emu, output, u32::MAX as u64));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[0, 0, data, output]),
                WSAEAFNOSUPPORT
            );
            assert_eq!(memory::read_pointer(&emu, output), Some(0));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[0, 0, 0, 0]),
                WSAEFAULT
            );
        }
    }
}

#[test]
fn address_conversion_pairs_share_ipv4_ipv6_and_error_contracts() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let binary = data + 0x200;
        let output = data + 0x300;
        for (parse, format, wide) in [
            ("inet_pton", "inet_ntop", false),
            ("InetPtonW", "InetNtopW", true),
        ] {
            for (family, text, expected) in [
                (AF_INET, "192.0.2.1", vec![192, 0, 2, 1]),
                (
                    AF_INET6,
                    "2001:db8::1",
                    vec![0x20, 1, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                ),
                (
                    AF_INET6,
                    "::ffff:192.0.2.1",
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 192, 0, 2, 1],
                ),
            ] {
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert!(
                    emu.maps
                        .write_bytes(data, &strings::encode_text(&format!("{text}\0"), wide))
                );
                assert!(emu.maps.write_bytes(binary, &[0xa5; 17]));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        parse,
                        &[family as u64, data, binary]
                    ),
                    1
                );
                assert_eq!(
                    emu.maps
                        .read_bytes_option(binary, expected.len())
                        .unwrap()
                        .as_ref(),
                    expected
                );
                assert_eq!(
                    emu.maps.read_byte(binary + expected.len() as u64),
                    Some(0xa5)
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert!(emu.maps.write_bytes(output, &[0xa5; 128]));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        format,
                        &[family as u64, binary, output, text.len() as u64]
                    ),
                    0
                );
                assert_eq!(emu.get_last_error(), WSAEINVAL);
                assert_eq!(emu.maps.read_byte(output), Some(0xa5));
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        format,
                        &[family as u64, binary, output, text.len() as u64 + 1]
                    ),
                    output
                );
                assert_eq!(strings::read_text(&emu, output, wide), text);
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            }
            for text in ["127.1", "01.2.3.4", "256.0.0.1", "bad"] {
                assert!(
                    emu.maps
                        .write_bytes(data, &strings::encode_text(&format!("{text}\0"), wide))
                );
                assert!(emu.maps.write_bytes(binary, &[0xa5; 16]));
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        parse,
                        &[AF_INET as u64, data, binary]
                    ),
                    0
                );
                assert_eq!(
                    emu.get_last_error(),
                    if wide { WSAEINVAL } else { ERROR_ACCESS_DENIED }
                );
                assert_eq!(
                    emu.maps.read_bytes_option(binary, 16).unwrap().as_ref(),
                    &[0xa5; 16]
                );
            }
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text("127.0.0.1\0", wide))
            );
            for (family, source, destination, error) in [
                (AF_UNSPEC, data, binary, WSAEAFNOSUPPORT),
                (AF_INET, 0, binary, WSAEFAULT),
                (AF_INET, data, 0, WSAEFAULT),
                (AF_INET, data, data + 0xfff, WSAEFAULT),
            ] {
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        parse,
                        &[family as u64, source, destination]
                    ),
                    SOCKET_ERROR
                );
                assert_eq!(emu.get_last_error(), error);
            }
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "ws2_32.dll",
                    format,
                    &[AF_UNSPEC as u64, binary, output, 64]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), WSAEAFNOSUPPORT);
        }
    }
}

#[test]
fn byte_order_helpers_use_guest_width_and_preserve_last_error() {
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        for (name, expected) in [
            ("htons", 0x7856),
            ("ntohs", 0x7856),
            ("htonl", 0x7856_3412),
            ("ntohl", 0x7856_3412),
        ] {
            emu.set_last_error(ERROR_ACCESS_DENIED);
            let value = if is64 {
                0xa5a5_a5a5_1234_5678
            } else {
                0x1234_5678
            };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "ws2_32.dll", name, &[value]),
                expected
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        }
    }
}

#[test]
fn fd_set_membership_uses_guest_layout_and_preserves_error_state() {
    const MARKER: u64 = 0x1234;
    // Applications can override FD_SETSIZE; 64 is only the SDK default.
    const SOCKET_COUNT: usize = 65;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = if is64 {
            size_of::<u64>()
        } else {
            size_of::<u32>()
        };
        let high_bits = if is64 { 1u64 << u32::BITS } else { 0 };
        let mut set = vec![0; width * (SOCKET_COUNT + 1)];
        set[..size_of::<u32>()].copy_from_slice(&(SOCKET_COUNT as u32).to_le_bytes());
        for (index, bytes) in set[width..].chunks_exact_mut(width).enumerate() {
            bytes.copy_from_slice(&(high_bits | (index as u64 + 1)).to_le_bytes()[..width]);
        }
        // Split fd_count across maps to cover checked reads at field boundaries.
        let split = data + 0x2000;
        emu.maps
            .create_map("fd_count_head", split, 2, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map(
                "fd_set_tail",
                split + 2,
                set.len() as u64 - 2,
                Permission::READ_WRITE,
            )
            .unwrap();
        for address in [data, split] {
            assert!(emu.maps.write_bytes(address, &set));
            for (socket, expected) in [
                (high_bits | 1, 1),
                (high_bits | 65, 1),
                (0, 0),
                (high_bits | 66, 0),
            ] {
                emu.set_last_error(MARKER);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "ws2_32.dll",
                        "__WSAFDIsSet",
                        &[socket, address]
                    ),
                    expected
                );
                assert_eq!(emu.get_last_error(), MARKER);
            }
            assert!(emu.maps.write_bytes(address, &0u32.to_le_bytes()));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "ws2_32.dll",
                    "__WSAFDIsSet",
                    &[high_bits | 1, address]
                ),
                0
            );
        }
    }
}

fn socket_call(emu: &mut emu::Emu, name: &str, arguments: &[u64]) -> u64 {
    helpers::call_winapi_export(emu, "ws2_32.dll", name, arguments)
}

fn create_socket(emu: &mut emu::Emu, family: u32, kind: u32) -> u64 {
    let socket = socket_call(emu, "socket", &[family as u64, kind as u64, 0]);
    assert!(emu.sockets.contains(socket));
    socket
}

fn write_socket_address(emu: &mut emu::Emu, address: u64, family: u32, port: u16) -> usize {
    let size = if family == AF_INET6 { 28 } else { 16 };
    let mut bytes = vec![0; size];
    bytes[..2].copy_from_slice(&(family as u16).to_le_bytes());
    bytes[2..4].copy_from_slice(&port.to_be_bytes());
    assert!(emu.maps.write_bytes(address, &bytes));
    size
}

#[test]
fn sockets_are_owned_by_the_emulator_and_survive_native_snapshots() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let (mut other, _) = helpers::prepare_winapi(is64);
        for name in ["socket", "WSASocketA", "WSASocketW"] {
            emu.set_last_error(ERROR_ACCESS_DENIED);
            let arguments = [AF_INET as u64, SOCK_STREAM as u64, 0, 0, 0, 0];
            let socket = socket_call(
                &mut emu,
                name,
                &arguments[..if name == "socket" { 3 } else { 6 }],
            );
            assert!(emu.sockets.contains(socket));
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            assert_eq!(
                socket_call(&mut other, "closesocket", &[socket]),
                SOCKET_ERROR
            );
            assert_eq!(other.get_last_error(), WSAENOTSOCK);
            emu = Serialization::deserialize(&Serialization::serialize(&emu));
            assert!(emu.sockets.contains(socket));
            assert_eq!(socket_call(&mut emu, "closesocket", &[socket]), 0);
            assert!(!emu.sockets.contains(socket));
            assert_eq!(
                socket_call(&mut emu, "closesocket", &[socket]),
                SOCKET_ERROR
            );
        }
        for handle in [0, 1, 2, INVALID_SOCKET] {
            assert_eq!(
                socket_call(&mut emu, "closesocket", &[handle]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAENOTSOCK);
        }
        let invalid = if is64 { INVALID_SOCKET } else { SOCKET_ERROR };
        for (family, kind, protocol, error) in [
            (AF_UNSPEC, SOCK_STREAM, 0, WSAEAFNOSUPPORT),
            (AF_INET, 0, 0, WSAESOCKTNOSUPPORT),
            (AF_INET, SOCK_DGRAM, IPPROTO_TCP, WSAEPROTONOSUPPORT),
        ] {
            assert_eq!(
                socket_call(
                    &mut emu,
                    "socket",
                    &[family as u64, kind as u64, protocol as u64]
                ),
                invalid
            );
            assert_eq!(emu.get_last_error(), error);
        }
        let socket = create_socket(&mut emu, AF_INET, SOCK_STREAM);
        // The x86 frame contains exactly three readable DWORDs: accept has no fourth argument.
        if !is64 {
            let frame = data + 0x1000 - 3 * size_of::<u32>() as u64;
            assert!(emu.maps.write_bytes(frame, &(socket as u32).to_le_bytes()));
            assert!(emu.maps.write_bytes(frame + 4, &0u32.to_le_bytes()));
            assert!(emu.maps.write_bytes(frame + 8, &0u32.to_le_bytes()));
            emu.set_sp(frame);
            helpers::invoke_winapi_export(&mut emu, "ws2_32.dll", "accept");
            assert!(!emu.api_policy.stopped());
            assert_eq!(emu.sp(), data + 0x1000);
            assert_eq!(emu.regs().get_eax(), SOCKET_ERROR);
            assert_eq!(emu.get_last_error(), WSAEINVAL);
        }
    }
}

#[test]
fn socket_addresses_and_accept_use_checked_in_out_lengths() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for family in [AF_INET, AF_INET6] {
            let socket = create_socket(&mut emu, family, SOCK_STREAM);
            let size = write_socket_address(&mut emu, data, family, 0);
            let output = data + 0x100;
            let length = data + 0x200;
            assert_eq!(socket_call(&mut emu, "listen", &[socket, 5]), SOCKET_ERROR);
            assert_eq!(emu.get_last_error(), WSAEINVAL);
            assert_eq!(
                socket_call(&mut emu, "bind", &[socket, data, size as u64 - 1]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEFAULT);
            assert_eq!(
                socket_call(&mut emu, "bind", &[socket, data, size as u64]),
                0
            );
            assert_eq!(
                socket_call(&mut emu, "bind", &[socket, data, size as u64]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEINVAL);
            assert!(emu.maps.write_bytes(output, &[0xa5; 32]));
            assert!(emu.maps.write_dword(length, size as u32 - 1));
            assert_eq!(
                socket_call(&mut emu, "getsockname", &[socket, output, length]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEFAULT);
            assert_eq!(emu.maps.read_byte(output), Some(0xa5));
            assert_eq!(emu.maps.read_dword(length), Some(size as u32 - 1));
            assert!(emu.maps.write_dword(length, size as u32));
            emu.set_last_error(ERROR_ACCESS_DENIED);
            assert_eq!(
                socket_call(&mut emu, "getsockname", &[socket, output, length]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            assert_eq!(emu.maps.read_word(output), Some(family as u16));
            assert_ne!(emu.maps.read_word(output + 2), Some(0));
            assert_eq!(emu.maps.read_dword(length), Some(size as u32));
            assert_eq!(emu.maps.read_byte(output + size as u64), Some(0xa5));
            assert_eq!(socket_call(&mut emu, "listen", &[socket, 5]), 0);
            assert!(emu.maps.write_dword(length, 1));
            assert_eq!(
                socket_call(&mut emu, "ioctlsocket", &[socket, FIONBIO as u64, length]),
                0
            );
            for name in ["accept", "WSAAccept"] {
                let arguments = [socket, output, length, 0, 0];
                let count = if name == "accept" { 3 } else { 5 };
                assert_eq!(
                    socket_call(&mut emu, name, &arguments[..count]),
                    if is64 { INVALID_SOCKET } else { SOCKET_ERROR }
                );
                assert_eq!(emu.get_last_error(), WSAEFAULT);
                assert!(emu.maps.write_dword(length, size as u32));
                assert!(emu.maps.write_bytes(output, &[0xa5; 32]));
                assert_eq!(
                    socket_call(&mut emu, name, &arguments[..count]),
                    if is64 { INVALID_SOCKET } else { SOCKET_ERROR }
                );
                assert_eq!(emu.get_last_error(), WSAEWOULDBLOCK);
                assert_eq!(emu.maps.read_byte(output), Some(0xa5));
                assert_eq!(emu.maps.read_dword(length), Some(size as u32));
                assert!(emu.maps.write_dword(length, 1));
            }
            assert_eq!(socket_call(&mut emu, "closesocket", &[socket]), 0);
        }
    }
}

#[test]
fn socket_options_round_trip_and_reject_short_or_invalid_outputs() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let socket = create_socket(&mut emu, AF_INET, SOCK_STREAM);
        let value = data + 0x100;
        let length = data + 0x200;
        // Include a LINGER structure: two little-endian u16 fields, not two native pointers.
        for (level, option, input, expected) in [
            (SOL_SOCKET, SO_KEEPALIVE, 42, 1),
            (SOL_SOCKET, SO_RCVTIMEO, 250, 250),
            (SOL_SOCKET, SO_LINGER, 5 << 16 | 1, 5 << 16 | 1),
            (IPPROTO_TCP, TCP_NODELAY, 7, 1),
        ] {
            assert!(emu.maps.write_dword(value, input));
            emu.set_last_error(ERROR_ACCESS_DENIED);
            assert_eq!(
                socket_call(
                    &mut emu,
                    "setsockopt",
                    &[socket, level as u64, option as u64, value, 4]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            emu = Serialization::deserialize(&Serialization::serialize(&emu));
            assert!(emu.maps.write_qword(value, u64::MAX));
            assert!(emu.maps.write_qword(length, u64::MAX));
            assert!(emu.maps.write_dword(length, 8));
            assert_eq!(
                socket_call(
                    &mut emu,
                    "getsockopt",
                    &[socket, level as u64, option as u64, value, length]
                ),
                0
            );
            assert_eq!(emu.maps.read_dword(value), Some(expected));
            assert_eq!(emu.maps.read_dword(value + 4), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(length), Some(4));
            assert_eq!(emu.maps.read_dword(length + 4), Some(u32::MAX));
        }
        for (name, length_arg) in [("setsockopt", 3), ("getsockopt", length)] {
            assert!(emu.maps.write_dword(value, 0xa5a5_a5a5));
            assert!(emu.maps.write_dword(length, 3));
            assert_eq!(
                socket_call(
                    &mut emu,
                    name,
                    &[
                        socket,
                        SOL_SOCKET as u64,
                        SO_KEEPALIVE as u64,
                        value,
                        length_arg
                    ]
                ),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEFAULT);
            assert_eq!(emu.maps.read_dword(value), Some(0xa5a5_a5a5));
        }
        assert_eq!(
            socket_call(
                &mut emu,
                "setsockopt",
                &[socket, SOL_SOCKET as u64, SO_TYPE as u64, value, 4]
            ),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAENOPROTOOPT);
        assert_eq!(
            socket_call(
                &mut emu,
                "getsockopt",
                &[
                    INVALID_SOCKET,
                    SOL_SOCKET as u64,
                    SO_TYPE as u64,
                    value,
                    length
                ]
            ),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAENOTSOCK);
        // Split a DWORD across adjacent mappings instead of assuming one-map fields.
        let split = data + 0x2000;
        emu.maps
            .create_map("option_head", split, 2, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map("option_tail", split + 2, 2, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_bytes(split, &1u32.to_le_bytes()));
        assert_eq!(
            socket_call(
                &mut emu,
                "setsockopt",
                &[socket, SOL_SOCKET as u64, SO_KEEPALIVE as u64, split, 4]
            ),
            0
        );
        assert_eq!(
            socket_call(&mut emu, "ioctlsocket", &[socket, FIONBIO as u64, split]),
            0
        );
        assert_eq!(
            socket_call(&mut emu, "ioctlsocket", &[socket, FIONREAD as u64, split]),
            0
        );
        assert_eq!(memory::read_array::<4>(&emu, split), Some([0; 4]));
    }
}

#[test]
fn offline_sockets_report_connection_and_buffer_state_without_fabricating_input() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let tcp = create_socket(&mut emu, AF_INET, SOCK_STREAM);
        let udp = create_socket(&mut emu, AF_INET, SOCK_DGRAM);
        let size = write_socket_address(&mut emu, data, AF_INET, 80) as u64;
        let buffer = data + 0x100;
        assert!(emu.maps.write_bytes(buffer, &[0xa5; 32]));
        assert_eq!(
            socket_call(&mut emu, "connect", &[tcp, data, size]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAECONNREFUSED);
        for name in ["send", "recv"] {
            assert_eq!(
                socket_call(&mut emu, name, &[0, buffer, 4, 0]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAENOTSOCK);
            assert_eq!(
                socket_call(&mut emu, name, &[tcp, buffer, 4, 0]),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAENOTCONN);
        }
        assert_eq!(socket_call(&mut emu, "connect", &[udp, data, size]), 0);
        for _ in 0..5 {
            assert_eq!(socket_call(&mut emu, "send", &[udp, buffer, 4, 0]), 4);
        }
        assert_eq!(socket_call(&mut emu, "send", &[udp, 0, 0, 0]), 0);
        assert_eq!(
            socket_call(&mut emu, "send", &[udp, buffer, u32::MAX as u64, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAEINVAL);
        assert_eq!(
            socket_call(&mut emu, "send", &[udp, data + 0xfff, 2, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAEFAULT);
        assert_eq!(
            socket_call(&mut emu, "WSAEventSelect", &[0, 0, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAENOTSOCK);
        assert_eq!(socket_call(&mut emu, "WSAEventSelect", &[udp, 0, 0]), 0);
        assert_eq!(
            socket_call(&mut emu, "recv", &[udp, buffer, 4, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAEWOULDBLOCK);
        assert!(emu.maps.write_dword(data, 1));
        assert_eq!(
            socket_call(&mut emu, "ioctlsocket", &[udp, FIONBIO as u64, data]),
            0
        );
        assert_eq!(
            socket_call(&mut emu, "recv", &[udp, buffer, 4, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAEWOULDBLOCK);
        assert_eq!(
            emu.maps.read_bytes_option(buffer, 32).unwrap().as_ref(),
            &[0xa5; 32]
        );
        assert_eq!(socket_call(&mut emu, "shutdown", &[udp, SD_SEND as u64]), 0);
        assert_eq!(
            socket_call(&mut emu, "send", &[udp, buffer, 4, 0]),
            SOCKET_ERROR
        );
        assert_eq!(emu.get_last_error(), WSAESHUTDOWN);
    }
}

#[test]
fn socket_waits_use_the_selected_policy_without_creating_a_peer() {
    for (is64, mode) in [false, true].into_iter().flat_map(|is64| {
        [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ]
        .map(|mode| (is64, mode))
    }) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.set_api_mode(mode);
        let socket = create_socket(&mut emu, AF_INET, SOCK_STREAM);
        let size = write_socket_address(&mut emu, data, AF_INET, 0) as u64;
        assert_eq!(socket_call(&mut emu, "bind", &[socket, data, size]), 0);
        assert_eq!(socket_call(&mut emu, "listen", &[socket, 5]), 0);
        let output = data + 0x100;
        let length = data + 0x200;
        assert!(emu.maps.write_bytes(output, &[0xa5; 32]));
        assert!(emu.maps.write_dword(length, size as u32));
        emu.set_last_error(ERROR_ACCESS_DENIED);
        let sp = emu.sp();
        if is64 {
            helpers::call_winapi64(
                &mut emu,
                |emu| helpers::invoke_winapi_export(emu, "ws2_32.dll", "accept"),
                &[socket, output, length],
            );
        } else {
            helpers::call_winapi32(
                &mut emu,
                |emu| helpers::invoke_winapi_export(emu, "ws2_32.dll", "accept"),
                &[socket as u32, output as u32, length as u32],
            );
        }
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(outcome.call.name, "accept");
        match mode {
            ApiMode::Strict => {
                assert_eq!(
                    outcome.disposition,
                    ApiDisposition::Stopped(ApiStopReason::StrictPolicy)
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert_eq!(
                    emu.sp(),
                    if is64 {
                        sp
                    } else {
                        sp - 3 * size_of::<u32>() as u64
                    }
                );
            }
            ApiMode::SimulateFail => {
                assert_eq!(outcome.disposition, ApiDisposition::SimulatedFailure);
                assert_eq!(emu.get_last_error(), WSAEOPNOTSUPP);
                assert_eq!(
                    emu.regs().rax,
                    if is64 { INVALID_SOCKET } else { SOCKET_ERROR }
                );
                assert_eq!(emu.sp(), sp);
            }
            ApiMode::SimulateSuccess => {
                assert_eq!(outcome.disposition, ApiDisposition::SimulatedSuccess);
                assert_eq!(emu.regs().rax, 0);
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert_eq!(emu.sp(), sp);
            }
        }
        assert!(!emu.sockets.contains(0));
        assert_eq!(
            emu.maps.read_bytes_option(output, 32).unwrap().as_ref(),
            &[0xa5; 32]
        );
        assert_eq!(emu.maps.read_dword(length), Some(size as u32));
    }
}

#[test]
fn socket_byte_order_writes_the_declared_output_width() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let lib = "ws2_32.dll";
        let sock = call(&mut emu, lib, "socket", &[2, 2, 17]);
        for (api, expected, size) in [
            ("WSAHtons", 0x4433, 2),
            ("WSANtohs", 0x4433, 2),
            ("WSAHtonl", 0x44332211, 4),
            ("WSANtohl", 0x44332211, 4),
        ] {
            emu.maps.write_qword(data, u64::MAX);
            assert_eq!(call(&mut emu, lib, api, &[sock, 0x11223344, data]), 0);
            assert_eq!(
                if size == 2 {
                    emu.maps.read_word(data).map(u32::from)
                } else {
                    emu.maps.read_dword(data)
                },
                Some(expected)
            );
            assert_eq!(emu.maps.read_byte(data + size), Some(u8::MAX));
            assert_eq!(call(&mut emu, lib, api, &[sock, 0, 0]), SOCKET_ERROR);
            assert_eq!(emu.get_last_error(), WSAEFAULT);
        }
    }
}

#[test]
fn winsock_events_share_the_windows_wait_state() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::write_pointer;
    use crate::windows::constants::process::{WAIT_OBJECT_0, WAIT_TIMEOUT};
    for is64 in [false, true] {
        let (mut emu, events) = prepare_winapi(is64);
        let library = "ws2_32.dll";
        let event = call(&mut emu, library, "WSACreateEvent", &[]);
        assert_ne!(event, 0);
        write_pointer(&mut emu, events, event);
        let args = [1, events, 0, 0, 0];
        assert_eq!(
            call(&mut emu, library, "WSAWaitForMultipleEvents", &args),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut emu, library, "WSASetEvent", &[event]), 1);
        for _ in 0..2 {
            assert_eq!(
                call(&mut emu, library, "WSAWaitForMultipleEvents", &args),
                WAIT_OBJECT_0
            );
        }
        assert_eq!(call(&mut emu, library, "WSAResetEvent", &[event]), 1);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "WaitForSingleObject", &[event, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut emu, library, "WSACloseEvent", &[event]), 1);
        assert_eq!(call(&mut emu, library, "WSACloseEvent", &[event]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        emu.set_api_mode(ApiMode::SimulateFail);
        assert_eq!(call(&mut emu, library, "WSAPoll", &[0; 3]), SOCKET_ERROR);
        assert_eq!(emu.get_last_error(), WSAEOPNOTSUPP);
    }
}

#[test]
fn textual_socket_endpoints_preserve_family_port_and_buffer_bounds() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let (address, length, out) = (data + 128, data + 256, data + 512);
        for (input, family, size) in [
            ("192.0.2.18:443", AF_INET, 16),
            ("[2001:db8::1]:80", AF_INET6, 28),
        ] {
            emu.maps.write_wide_string(data, input);
            emu.maps.write_dword(length, size);
            assert_eq!(
                call(
                    &mut emu,
                    "ws2_32.dll",
                    "WSAStringToAddressW",
                    &[data, family.into(), 0, address, length]
                ),
                0
            );
            assert_eq!(emu.maps.read_word(address), Some(family as u16));
            emu.maps.write_dword(length, 1);
            emu.maps.write_byte(out, 0xcc);
            assert_eq!(
                call(
                    &mut emu,
                    "ws2_32.dll",
                    "WSAAddressToStringA",
                    &[address, size.into(), 0, out, length]
                ),
                SOCKET_ERROR
            );
            assert_eq!(emu.get_last_error(), WSAEFAULT);
            assert_eq!(emu.maps.read_byte(out), Some(0xcc));
            assert_eq!(
                call(
                    &mut emu,
                    "ws2_32.dll",
                    "WSAAddressToStringA",
                    &[address, size.into(), 0, out, length]
                ),
                0
            );
            assert_eq!(emu.maps.read_string(out), input);
        }
    }
}
