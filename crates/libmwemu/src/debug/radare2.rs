//! Interactive radare2 access to the live emulator through GDB remote protocol.

use std::io;
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use crate::arch::Arch;
use crate::debug::gdb::GdbServer;
use crate::emu::Emu;

/// Open radare2 on live guest memory and registers, without creating a dump.
pub fn spawn(emu: &mut Emu, address: u64) -> io::Result<()> {
    run(emu, address, &mut Command::new("radare2"))
}

fn run(emu: &mut Emu, address: u64, command: &mut Command) -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    listener.set_nonblocking(true)?;
    let port = listener.local_addr()?.port();
    let (arch, bits) = match emu.cfg.arch {
        Arch::X86 => ("x86", "32"),
        Arch::X86_64 => ("x86", "64"),
        Arch::Aarch64 => ("arm", "64"),
    };
    let mut child = command
        .args([
            "-n",
            "-d",
            "-a",
            arch,
            "-b",
            bits,
            "-s",
            &format!("0x{address:x}"),
            "-e",
            "dbg.exitkills=false",
            &format!("gdb://127.0.0.1:{port}"),
        ])
        .spawn()?;

    let connection = accept_child(&listener, &mut child);
    drop(listener);
    let result = connection.and_then(|stream| {
        GdbServer::new(port, emu.cfg.arch)
            .serve_connection(emu, stream)
            .map_err(io::Error::other)
    });
    if result.is_err() {
        // A failed connection must not leave a child waiting for a dead server.
        let _ = child.kill();
    }
    let status = child.wait()?;
    result?;
    if !status.success() {
        return Err(io::Error::other(format!("radare2 exited with {status}")));
    }
    Ok(())
}

fn accept_child(listener: &TcpListener, child: &mut Child) -> io::Result<TcpStream> {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match listener.accept() {
            Ok((stream, _)) => return Ok(stream),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
        if let Some(status) = child.try_wait()? {
            return Err(io::Error::other(format!(
                "radare2 exited before connecting: {status}"
            )));
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "radare2 did not connect within 30 seconds",
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maps::mem64::Permission;

    #[test]
    fn missing_radare2_returns_an_error() {
        let mut emu = crate::emu64();
        let mut command = Command::new("/nonexistent/mwemu-test-radare2");
        let result = run(&mut emu, 0x1000, &mut command);
        assert!(matches!(result, Err(error) if error.kind() == io::ErrorKind::NotFound));
    }

    #[cfg(unix)]
    #[test]
    fn child_exit_before_connect_does_not_hang() {
        let mut emu = crate::emu64();
        let result = run(&mut emu, 0x1000, &mut Command::new("true"));
        assert!(matches!(result, Err(error) if error.to_string().contains("before connecting")));
    }

    #[test]
    #[ignore = "requires an installed radare2 with the GDB plugin; no downloads"]
    fn installed_radare2_edits_live_memory_and_registers() -> Result<(), Box<dyn std::error::Error>>
    {
        for arch in [Arch::X86, Arch::X86_64, Arch::Aarch64] {
            let mut emu = match arch {
                Arch::X86 => crate::emu32(),
                Arch::X86_64 => crate::emu64(),
                Arch::Aarch64 => crate::emu_aarch64(),
            };
            for (name, address) in [("code", 0x1000), ("data", 0x2000)] {
                emu.maps
                    .create_map(name, address, 0x100, Permission::READ_WRITE_EXECUTE)?;
            }
            emu.set_pc(0x1000);
            emu.enable_console();
            assert!(emu.maps.write_bytes(0x1000, &[0xde, 0xad, 0xbe, 0xef]));
            let register = match arch {
                Arch::X86 => "eax",
                Arch::X86_64 => "rax",
                Arch::Aarch64 => "x0",
            };
            let script = format!(
                "y 4 @ 0x1000; yy @ 0x2000; wx 11223344 @ 0x1000; dr {register}=0x12345678; :detach; q!"
            );
            let mut command = Command::new("radare2");
            command.args(["-NN", "-q", "-c", &script]);
            run(&mut emu, 0x1000, &mut command)?;
            assert_eq!(emu.maps.read_dword(0x1000), Some(0x44332211));
            assert_eq!(emu.maps.read_dword(0x2000), Some(0xefbeadde));
            let value = if arch == Arch::Aarch64 {
                emu.regs_aarch64().x[0]
            } else {
                emu.regs().rax
            };
            assert_eq!(value, 0x12345678);
            assert!(emu.cfg.console_enabled);
            assert_eq!(emu.pc(), 0x1000);
        }
        Ok(())
    }
}
