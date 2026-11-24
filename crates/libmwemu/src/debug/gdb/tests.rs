use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use super::GdbServer;
use super::target::MwemuTarget64;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use gdbstub::target::ext::base::singlethread::SingleThreadBase;

fn emulator(arch: Arch) -> Result<Emu, Box<dyn std::error::Error>> {
    let mut emu = match arch {
        Arch::X86 => crate::emu32(),
        Arch::X86_64 => crate::emu64(),
        Arch::Aarch64 => crate::emu_aarch64(),
    };
    for (name, address, size) in [
        ("first", 0x1000, 4),
        ("second", 0x1004, 4),
        ("far", 0x2000, 16),
    ] {
        emu.maps
            .create_map(name, address, size, Permission::READ_WRITE_EXECUTE)?;
    }
    emu.maps
        .create_map("readonly", 0x3000, 4, Permission::READ_EXECUTE)?;
    assert!(emu.maps.write_bytes(0x1000, &[1, 2, 3, 4, 5, 6, 7, 8]));
    emu.set_pc(0x1000);
    emu.enable_console();
    Ok(emu)
}

fn connection_pair() -> io::Result<(TcpStream, TcpStream)> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let client = TcpStream::connect(listener.local_addr()?)?;
    let (server, _) = listener.accept()?;
    for stream in [&client, &server] {
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        stream.set_nodelay(true)?;
    }
    Ok((client, server))
}

fn request(stream: &mut TcpStream, payload: &str) -> io::Result<String> {
    let checksum = payload.bytes().fold(0u8, u8::wrapping_add);
    write!(stream, "${payload}#{checksum:02x}")?;
    let mut byte = [0];
    stream.read_exact(&mut byte)?;
    assert_eq!(byte[0], b'+');
    stream.read_exact(&mut byte)?;
    assert_eq!(byte[0], b'$');
    let mut raw = Vec::new();
    loop {
        stream.read_exact(&mut byte)?;
        if byte[0] == b'#' {
            break;
        }
        raw.push(byte[0]);
    }
    let mut checksum = [0; 2];
    stream.read_exact(&mut checksum)?;
    let expected = raw.iter().copied().fold(0u8, u8::wrapping_add);
    assert_eq!(checksum.as_slice(), format!("{expected:02x}").as_bytes());
    stream.write_all(b"+")?;
    // Decode GDB run-length encoding: `*X` repeats the previous byte (X - 29) times.
    let mut response = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'*' && i > 0 && i + 1 < raw.len() {
            let count = raw[i + 1].wrapping_sub(29);
            let prev = *response.last().unwrap_or(&b'0');
            for _ in 0..count {
                response.push(prev);
            }
            i += 2;
        } else {
            response.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8(response).map_err(io::Error::other)
}

#[test]
fn remote_memory_and_registers_are_live_on_all_architectures()
-> Result<(), Box<dyn std::error::Error>> {
    for arch in [Arch::X86, Arch::X86_64, Arch::Aarch64] {
        let mut emu = emulator(arch)?;
        let (mut client, server) = connection_pair()?;
        let peer = std::thread::spawn(move || -> io::Result<()> {
            assert_eq!(request(&mut client, "m1002,4")?, "03040506");
            assert_eq!(request(&mut client, "M1002,4:aabbccdd")?, "OK");
            assert_eq!(request(&mut client, "m1000,8")?, "0102aabbccdd0708");
            assert_eq!(request(&mut client, "M2000,4:11223344")?, "OK");
            assert_eq!(request(&mut client, "m2000,4")?, "11223344");
            // A failed write must report an error without partially changing memory.
            for command in ["M1006,4:ffffffff", "M3000,1:ff", "M4000,1:ff"] {
                assert!(request(&mut client, command)?.starts_with('E'));
            }
            assert!(request(&mut client, "m1006,4")?.starts_with('E'));
            assert_eq!(request(&mut client, "m1006,2")?, "0708");
            assert!(request(&mut client, "m4000,1")?.starts_with('E'));
            let value = if arch == Arch::X86 {
                "78563412"
            } else {
                "7856341200000000"
            };
            assert_eq!(request(&mut client, &format!("P0={value}"))?, "OK");
            assert_eq!(request(&mut client, "p0")?, value);
            assert_eq!(request(&mut client, "D")?, "OK");
            Ok(())
        });
        let result = GdbServer::new(0, arch).serve_connection(&mut emu, server);
        peer.join()
            .map_err(|_| io::Error::other("GDB test client failed"))??;
        result?;
        assert!(emu.cfg.console_enabled);
        assert_eq!(emu.pc(), 0x1000);
        assert_eq!(emu.maps.read_dword(0x2000), Some(0x44332211));
        let register = if arch == Arch::Aarch64 {
            emu.regs_aarch64().x[0]
        } else {
            emu.regs().rax
        };
        assert_eq!(register, 0x12345678);
    }
    Ok(())
}

#[test]
fn remote_memory_rejects_overflow_and_preserves_permissions()
-> Result<(), Box<dyn std::error::Error>> {
    let mut emu = emulator(Arch::X86_64)?;
    let mut data = [0xaa; 4];
    assert!(
        MwemuTarget64::new(&mut emu)
            .read_addrs(u64::MAX - 1, &mut data)
            .is_err()
    );
    assert_eq!(data, [0xaa; 4]);
    assert!(
        MwemuTarget64::new(&mut emu)
            .write_addrs(u64::MAX - 1, &data)
            .is_err()
    );
    assert!(
        MwemuTarget64::new(&mut emu)
            .write_addrs(0x3000, &data)
            .is_err()
    );
    assert_eq!(emu.maps.read_dword(0x3000), Some(0));
    assert_eq!(
        emu.maps.get_mem("readonly").permission(),
        Permission::READ_EXECUTE
    );
    // Guest code writes must continue to invalidate decoded instructions.
    let mut writes = Vec::new();
    emu.maps.drain_code_writes(&mut writes);
    assert!(
        MwemuTarget64::new(&mut emu)
            .write_addrs(0x2000, &data)
            .is_ok()
    );
    assert!(emu.maps.has_code_writes());
    Ok(())
}

#[test]
fn console_settings_survive_client_disconnects() -> Result<(), Box<dyn std::error::Error>> {
    for enabled in [false, true] {
        let mut emu = emulator(Arch::X86_64)?;
        emu.cfg.console_enabled = enabled;
        let (client, server) = connection_pair()?;
        drop(client);
        assert!(
            GdbServer::new(0, emu.cfg.arch)
                .serve_connection(&mut emu, server)
                .is_err()
        );
        assert_eq!(emu.cfg.console_enabled, enabled);
    }
    Ok(())
}
