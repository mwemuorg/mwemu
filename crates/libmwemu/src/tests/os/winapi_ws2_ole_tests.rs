use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;
use crate::windows::constants::system::*;

use crate::*;

// ─── ws2_32 64-bit ───

#[test]
fn wsa_startup_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::WSAStartup, &[0x0202, 0]);
    assert_eq!(ret, WSAEFAULT, "WSAStartup requires writable WSADATA");
}

#[test]
fn socket_create_64() {
    helpers::setup();
    let mut emu = emu64();
    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );
    assert_ne!(sock, 0, "socket() should return a non-zero handle");
}

#[test]
fn htons_64() {
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
fn closesocket_64() {
    helpers::setup();
    let mut emu = emu64();
    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket should return 0");
}

#[test]
fn connect_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );

    let sockaddr_base = 0x30000u64;
    emu.maps
        .create_map("sockaddr", sockaddr_base, 0x1000, Permission::READ_WRITE);
    emu.maps.write_word(sockaddr_base, AF_INET as u16); // family
    emu.maps.write_word(sockaddr_base + 2, 80u16.to_be()); // port
    emu.maps
        .write_dword(sockaddr_base + 4, u32::from_le_bytes([127, 0, 0, 1]));

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::connect,
        &[sock, sockaddr_base, 16],
    );
    assert_eq!(ret, 0, "connect on valid socket should return 0 (offline)");
}

#[test]
fn bind_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );

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
fn listen_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::listen, &[sock, 5]);
    assert_eq!(ret, 0, "listen on valid socket should return 0");
}

#[test]
fn accept_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );

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
fn setsockopt_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );

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
fn getsockopt_valid_socket_64() {
    helpers::setup();
    let mut emu = emu64();

    let sock = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );

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
fn socket_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let s1 = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );
    let s2 = helpers::call_winapi64(
        &mut emu,
        winapi64::ws2_32::socket,
        &[AF_INET as u64, SOCK_STREAM as u64, IPPROTO_TCP as u64],
    );
    assert_ne!(s1, s2, "two sockets should have different handles");

    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[s1]);
    assert_eq!(ret, 0);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ws2_32::closesocket, &[s2]);
    assert_eq!(ret, 0);
}

// ─── ws2_32 32-bit ───

#[test]
fn wsa_startup_32() {
    helpers::setup();
    let mut emu = emu32();
    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::WSAStartup, &[0x0202, 0]);
    assert_eq!(
        ret as u64, WSAEFAULT,
        "WSAStartup requires writable WSADATA"
    );
}

#[test]
fn socket_create_32() {
    helpers::setup();
    let mut emu = emu32();
    let sock = helpers::call_winapi32(
        &mut emu,
        winapi32::ws2_32::socket,
        &[AF_INET, SOCK_STREAM, IPPROTO_TCP],
    );
    assert_ne!(sock, 0, "socket() 32-bit should return a non-zero handle");
}

#[test]
fn htons_32() {
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
fn connect_valid_socket_32() {
    helpers::setup();
    let mut emu = emu32();

    let sock = helpers::call_winapi32(
        &mut emu,
        winapi32::ws2_32::socket,
        &[AF_INET, SOCK_STREAM, IPPROTO_TCP],
    );

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
fn closesocket_32() {
    helpers::setup();
    let mut emu = emu32();
    let sock = helpers::call_winapi32(
        &mut emu,
        winapi32::ws2_32::socket,
        &[AF_INET, SOCK_STREAM, IPPROTO_TCP],
    );
    let ret = helpers::call_winapi32(&mut emu, winapi32::ws2_32::closesocket, &[sock]);
    assert_eq!(ret, 0, "closesocket 32-bit should return 0");
}

// ─── ole32 64-bit ───

#[test]
fn co_initialize_ex_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoInitializeEx, &[0, 0]);
    assert_eq!(ret, 0, "CoInitializeEx should return S_OK (0)");
}

#[test]
fn co_create_instance_class_not_reg_64() {
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
        ret, REGDB_E_CLASSNOTREG,
        "CoCreateInstance should return REGDB_E_CLASSNOTREG"
    );
    let ppv = emu.maps.read_qword(buf + 0x100).unwrap();
    assert_eq!(ppv, 0, "ppv should be set to NULL");
}

#[test]
fn co_task_mem_alloc_64() {
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
fn co_task_mem_free_64() {
    helpers::setup();
    let mut emu = emu64();
    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x100]);
    assert_ne!(ptr, 0);
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);
    assert_eq!(ret, 0, "CoTaskMemFree should return 0");
}

#[test]
fn co_task_mem_free_null_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[0]);
    assert_eq!(ret, 0, "CoTaskMemFree(NULL) should not crash");
}

#[test]
fn co_uninitialize_64() {
    helpers::setup();
    let mut emu = emu64();
    let ret = helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
    assert_eq!(ret, 0, "CoUninitialize should return 0");
}

#[test]
fn co_task_mem_alloc_free_cycle_64() {
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
fn sys_alloc_string_len_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 32]);
    assert_ne!(bstr, 0, "SysAllocStringLen should return non-zero BSTR");
    assert!(emu.maps.is_mapped(bstr), "BSTR pointer should be mapped");
}

#[test]
fn sys_free_string_64() {
    helpers::setup();
    let mut emu = emu64();
    let bstr = helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysAllocStringLen, &[0, 16]);
    assert_ne!(bstr, 0);
    helpers::call_winapi64(&mut emu, winapi64::oleaut32::SysFreeString, &[bstr]);
}

#[test]
fn variant_clear_64() {
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
fn sys_alloc_string_64() {
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
fn sys_string_len_64() {
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
fn sys_string_byte_len_64() {
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
fn variant_init_64() {
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

#[test]
fn com_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoInitializeEx,
        &[0, COINIT_APARTMENTTHREADED as u64],
    );
    assert_eq!(ret, 0, "CoInitializeEx must return S_OK");

    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "CoTaskMemAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xBAADF00D);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xBAADF00D);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
}

#[test]
fn co_create_instance_64() {
    helpers::setup();
    let mut emu = emu64();

    let clsid_addr = 0x100000u64;
    let iid_addr = 0x100020u64;
    let ppv_addr = 0x100040u64;
    emu.maps
        .create_map("com_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(ppv_addr, 0xDEAD);

    // CoCreateInstance(rclsid, pUnkOuter, dwClsContext, riid, ppv)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoCreateInstance,
        &[clsid_addr, 0, 1, iid_addr, ppv_addr],
    );
    assert_eq!(
        ret, REGDB_E_CLASSNOTREG,
        "CoCreateInstance must return REGDB_E_CLASSNOTREG"
    );
    let ppv = emu.maps.read_qword(ppv_addr).unwrap();
    assert_eq!(ppv, 0, "ppv must be set to NULL");
}

#[test]
fn automation_conversions_use_typed_arguments_and_exact_output_widths() {
    struct Conversion {
        api: &'static str,
        value: f64,
        single: bool,
        output: Vec<u8>,
    }
    helpers::setup();
    let cases = [
        Conversion {
            api: "VarI2FromR8",
            value: 2.5,
            single: false,
            output: 2i16.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarI4FromR8",
            value: 3.5,
            single: false,
            output: 4i32.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarI8FromR8",
            value: -4_294_967_297.0,
            single: false,
            output: (-4_294_967_297i64).to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarUI1FromR4",
            value: 254.5,
            single: true,
            output: vec![254],
        },
        Conversion {
            api: "VarBoolFromR8",
            value: -1.5,
            single: false,
            output: (-1i16).to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarR4FromR8",
            value: 1.25,
            single: false,
            output: 1.25f32.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarR8FromR4",
            value: 1.5,
            single: true,
            output: 1.5f64.to_le_bytes().to_vec(),
        },
        Conversion {
            api: "VarCyFromR8",
            value: 1.25,
            single: false,
            output: 12500i64.to_le_bytes().to_vec(),
        },
    ];
    const CANARY: u8 = 0xa5;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for case in &cases {
            emu.maps.fill_range(data, CANARY, 32);
            let arguments = if is64 {
                emu.regs_mut().xmm0 = if case.single {
                    (case.value as f32).to_bits() as u128
                } else {
                    case.value.to_bits() as u128
                };
                vec![0x1234, data]
            } else {
                let mut words: Vec<u64> = if case.single {
                    vec![(case.value as f32).to_bits() as u64]
                } else {
                    helpers::f64_arguments32(&[case.value])
                        .into_iter()
                        .map(u64::from)
                        .collect()
                };
                words.push(data);
                words
            };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "oleaut32.dll", case.api, &arguments),
                S_OK,
                "{}",
                case.api
            );
            for (index, &byte) in case.output.iter().enumerate() {
                assert_eq!(
                    emu.maps.read_byte(data + index as u64),
                    Some(byte),
                    "{}",
                    case.api
                );
            }
            assert_eq!(
                emu.maps.read_byte(data + case.output.len() as u64),
                Some(CANARY),
                "{} output overrun",
                case.api
            );
        }
        emu.maps.fill_range(data, CANARY, 16);
        let arguments = if is64 {
            emu.regs_mut().xmm0 = f64::NAN.to_bits() as u128;
            vec![0, data]
        } else {
            let mut words: Vec<u64> = helpers::f64_arguments32(&[f64::NAN])
                .into_iter()
                .map(u64::from)
                .collect();
            words.push(data);
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarI4FromR8", &arguments),
            DISP_E_OVERFLOW
        );
        assert_eq!(
            emu.maps.read_dword(data),
            Some(u32::from_le_bytes([CANARY; 4]))
        );
    }
}

#[test]
fn automation_mixed_fp_arguments_math_and_dates() {
    helpers::setup();
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let power_arguments = if is64 {
            emu.regs_mut().xmm0 = 2.0f64.to_bits() as u128;
            emu.regs_mut().xmm1 = 5.0f64.to_bits() as u128;
            vec![0, 0, data]
        } else {
            let mut words: Vec<u64> = helpers::f64_arguments32(&[2.0, 5.0])
                .into_iter()
                .map(u64::from)
                .collect();
            words.push(data);
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarR8Pow", &power_arguments),
            S_OK
        );
        assert_eq!(emu.maps.read_qword(data), Some(32.0f64.to_bits()));
        let compare_arguments = if is64 {
            emu.regs_mut().xmm0 = 1.5f32.to_bits() as u128;
            emu.regs_mut().xmm1 = 2.5f64.to_bits() as u128;
            vec![0, 0]
        } else {
            let mut words = vec![1.5f32.to_bits() as u64];
            words.extend(helpers::f64_arguments32(&[2.5]).into_iter().map(u64::from));
            words
        };
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "oleaut32.dll", "VarR4CmpR8", &compare_arguments),
            0
        );
        for (value, year, month, day, hour) in [(2.5, 1900, 1, 1, 12), (-1.25, 1899, 12, 29, 6)] {
            emu.maps.fill_range(data, 0xa5, 32);
            let arguments = if is64 {
                emu.regs_mut().xmm0 = f64::to_bits(value) as u128;
                vec![0, data]
            } else {
                let mut words: Vec<u64> = helpers::f64_arguments32(&[value])
                    .into_iter()
                    .map(u64::from)
                    .collect();
                words.push(data);
                words
            };
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "oleaut32.dll",
                    "VariantTimeToSystemTime",
                    &arguments
                ),
                1
            );
            assert_eq!(emu.maps.read_word(data), Some(year));
            assert_eq!(emu.maps.read_word(data + 2), Some(month));
            assert_eq!(emu.maps.read_word(data + 6), Some(day));
            assert_eq!(emu.maps.read_word(data + 8), Some(hour));
            assert_eq!(emu.maps.read_word(data + 16), Some(0xa5a5));
        }
    }
}
