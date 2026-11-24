//! GDB Remote Debugging Support for mwemu
//!
//! This module provides GDB remote debugging protocol support, allowing debuggers
//! like GDB and IDA Pro to connect and debug emulated binaries.

mod breakpoints;
mod registers;
mod target;
mod target_xml;
#[cfg(test)]
mod tests;

use std::io;
use std::marker::PhantomData;
use std::net::{TcpListener, TcpStream};

use gdbstub::common::Signal;
use gdbstub::conn::ConnectionExt;
use gdbstub::stub::run_blocking::{BlockingEventLoop, Event, WaitForStopReasonError};
use gdbstub::stub::{DisconnectReason, GdbStub, SingleThreadStopReason};
use gdbstub::target::Target;

use crate::arch::Arch;
use crate::emu::Emu;
use target::{MwemuTarget32, MwemuTarget64, MwemuTargetAarch64};

/// Error type for GDB server operations
#[derive(Debug)]
pub enum GdbServerError {
    Io(io::Error),
    Connection(String),
    Protocol(String),
}

impl std::fmt::Display for GdbServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GdbServerError::Io(e) => write!(f, "IO error: {}", e),
            GdbServerError::Connection(s) => write!(f, "Connection error: {}", s),
            GdbServerError::Protocol(s) => write!(f, "Protocol error: {}", s),
        }
    }
}

impl std::error::Error for GdbServerError {}

impl From<io::Error> for GdbServerError {
    fn from(e: io::Error) -> Self {
        GdbServerError::Io(e)
    }
}

/// GDB Server for mwemu
pub struct GdbServer {
    port: u16,
    arch: Arch,
}

impl GdbServer {
    /// Create a new GDB server instance
    pub fn new(port: u16, arch: Arch) -> Self {
        Self { port, arch }
    }

    /// Start the GDB server and wait for a connection
    pub fn run(&mut self, emu: &mut Emu) -> Result<(), GdbServerError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        let port = listener.local_addr()?.port();
        log::info!("GDB server listening on port {}...", port);
        log::info!("Connect with: target remote localhost:{}", port);

        let (stream, addr) = listener.accept()?;
        log::info!("GDB client connected from {}", addr);

        self.serve_connection(emu, stream)
    }

    /// Serve an already connected client. Emulation stays stopped until the
    /// client explicitly steps or continues it. Console settings are restored
    /// when the session ends, including on protocol errors.
    pub fn serve_connection(&self, emu: &mut Emu, stream: TcpStream) -> Result<(), GdbServerError> {
        let console_enabled = std::mem::replace(&mut emu.cfg.console_enabled, false);
        let result = match self.arch {
            Arch::Aarch64 => run_aarch64(emu, stream),
            Arch::X86_64 => run_64bit(emu, stream),
            Arch::X86 => run_32bit(emu, stream),
        };
        emu.cfg.console_enabled = console_enabled;
        result
    }
}

fn run_64bit(emu: &mut Emu, stream: TcpStream) -> Result<(), GdbServerError> {
    let mut target = MwemuTarget64::new(emu);

    let gdb = GdbStub::new(stream);

    match gdb.run_blocking::<MwemuEventLoop64<'_>>(&mut target) {
        Ok(disconnect_reason) => {
            match disconnect_reason {
                DisconnectReason::Disconnect => {
                    log::info!("GDB client disconnected");
                }
                DisconnectReason::TargetExited(code) => {
                    log::info!("Target exited with code {}", code);
                }
                DisconnectReason::TargetTerminated(sig) => {
                    log::info!("Target terminated with signal {:?}", sig);
                }
                DisconnectReason::Kill => {
                    log::info!("GDB sent kill command");
                }
            }
            Ok(())
        }
        Err(e) => Err(GdbServerError::Protocol(format!("GDB error: {:?}", e))),
    }
}

fn run_aarch64(emu: &mut Emu, stream: TcpStream) -> Result<(), GdbServerError> {
    let mut target = MwemuTargetAarch64::new(emu);

    let gdb = GdbStub::new(stream);

    match gdb.run_blocking::<MwemuEventLoopAarch64<'_>>(&mut target) {
        Ok(disconnect_reason) => {
            match disconnect_reason {
                DisconnectReason::Disconnect => {
                    log::info!("GDB client disconnected");
                }
                DisconnectReason::TargetExited(code) => {
                    log::info!("Target exited with code {}", code);
                }
                DisconnectReason::TargetTerminated(sig) => {
                    log::info!("Target terminated with signal {:?}", sig);
                }
                DisconnectReason::Kill => {
                    log::info!("GDB sent kill command");
                }
            }
            Ok(())
        }
        Err(e) => Err(GdbServerError::Protocol(format!("GDB error: {:?}", e))),
    }
}

fn run_32bit(emu: &mut Emu, stream: TcpStream) -> Result<(), GdbServerError> {
    let mut target = MwemuTarget32::new(emu);

    let gdb = GdbStub::new(stream);

    match gdb.run_blocking::<MwemuEventLoop32<'_>>(&mut target) {
        Ok(disconnect_reason) => {
            match disconnect_reason {
                DisconnectReason::Disconnect => {
                    log::info!("GDB client disconnected");
                }
                DisconnectReason::TargetExited(code) => {
                    log::info!("Target exited with code {}", code);
                }
                DisconnectReason::TargetTerminated(sig) => {
                    log::info!("Target terminated with signal {:?}", sig);
                }
                DisconnectReason::Kill => {
                    log::info!("GDB sent kill command");
                }
            }
            Ok(())
        }
        Err(e) => Err(GdbServerError::Protocol(format!("GDB error: {:?}", e))),
    }
}

fn poll_incoming(conn: &mut TcpStream) -> io::Result<Option<u8>> {
    let pending = ConnectionExt::peek(conn);
    // gdbstub's TCP peek leaves the stream nonblocking, including for stop replies.
    conn.set_nonblocking(false)?;
    if pending?.is_some() {
        conn.read().map(Some)
    } else {
        Ok(None)
    }
}

/// Event loop implementation for 64-bit targets
struct MwemuEventLoop64<'a>(PhantomData<&'a mut Emu>);

impl<'a> BlockingEventLoop for MwemuEventLoop64<'a> {
    type Target = MwemuTarget64<'a>;
    type Connection = TcpStream;
    type StopReason = SingleThreadStopReason<u64>;

    fn wait_for_stop_reason(
        target: &mut Self::Target,
        conn: &mut Self::Connection,
    ) -> Result<
        Event<Self::StopReason>,
        WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as gdbstub::conn::Connection>::Error,
        >,
    > {
        loop {
            if let Some(byte) = poll_incoming(conn).map_err(WaitForStopReasonError::Connection)? {
                return Ok(Event::IncomingData(byte));
            }

            // Check if we hit a breakpoint before executing
            let rip = target.emu.regs().rip;
            if target.emu.bp.is_bp(rip) && !target.single_step {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }

            // Single step mode - return after one instruction
            if target.single_step {
                target.single_step = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::DoneStep));
            }

            // Execute one instruction
            let result = target.emu.step();
            if !result {
                // Emulation ended
                return Ok(Event::TargetStopped(SingleThreadStopReason::Terminated(
                    Signal::SIGTERM,
                )));
            }

            // Check if a library was loaded during this step
            if target.emu.library_loaded {
                target.emu.library_loaded = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::Library(())));
            }

            // Check for memory watchpoints
            for mem_op in &target.emu.memory_operations {
                if target.emu.bp.is_bp_mem_read(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Read,
                        addr: mem_op.address,
                    }));
                }
                if target.emu.bp.is_bp_mem_write_addr(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Write,
                        addr: mem_op.address,
                    }));
                }
            }

            // Check for breakpoint after step
            let rip = target.emu.regs().rip;
            if target.emu.bp.is_bp(rip) {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }
        }
    }

    fn on_interrupt(
        _target: &mut Self::Target,
    ) -> Result<Option<Self::StopReason>, <Self::Target as Target>::Error> {
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}

/// Event loop implementation for AArch64 targets
struct MwemuEventLoopAarch64<'a>(PhantomData<&'a mut Emu>);

impl<'a> BlockingEventLoop for MwemuEventLoopAarch64<'a> {
    type Target = MwemuTargetAarch64<'a>;
    type Connection = TcpStream;
    type StopReason = SingleThreadStopReason<u64>;

    fn wait_for_stop_reason(
        target: &mut Self::Target,
        conn: &mut Self::Connection,
    ) -> Result<
        Event<Self::StopReason>,
        WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as gdbstub::conn::Connection>::Error,
        >,
    > {
        loop {
            if let Some(byte) = poll_incoming(conn).map_err(WaitForStopReasonError::Connection)? {
                return Ok(Event::IncomingData(byte));
            }

            // Check if we hit a breakpoint before executing
            let pc = target.emu.regs_aarch64().pc;
            if target.emu.bp.is_bp(pc) && !target.single_step {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }

            // Single step mode - return after one instruction
            if target.single_step {
                target.single_step = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::DoneStep));
            }

            // Execute one instruction
            let result = target.emu.step();
            if !result {
                return Ok(Event::TargetStopped(SingleThreadStopReason::Terminated(
                    Signal::SIGTERM,
                )));
            }

            // Check if a library was loaded during this step
            if target.emu.library_loaded {
                target.emu.library_loaded = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::Library(())));
            }

            // Check for memory watchpoints
            for mem_op in &target.emu.memory_operations {
                if target.emu.bp.is_bp_mem_read(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Read,
                        addr: mem_op.address,
                    }));
                }
                if target.emu.bp.is_bp_mem_write_addr(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Write,
                        addr: mem_op.address,
                    }));
                }
            }

            // Check for breakpoint after step
            let pc = target.emu.regs_aarch64().pc;
            if target.emu.bp.is_bp(pc) {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }
        }
    }

    fn on_interrupt(
        _target: &mut Self::Target,
    ) -> Result<Option<Self::StopReason>, <Self::Target as Target>::Error> {
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}

/// Event loop implementation for 32-bit targets
struct MwemuEventLoop32<'a>(PhantomData<&'a mut Emu>);

impl<'a> BlockingEventLoop for MwemuEventLoop32<'a> {
    type Target = MwemuTarget32<'a>;
    type Connection = TcpStream;
    type StopReason = SingleThreadStopReason<u32>;

    fn wait_for_stop_reason(
        target: &mut Self::Target,
        conn: &mut Self::Connection,
    ) -> Result<
        Event<Self::StopReason>,
        WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as gdbstub::conn::Connection>::Error,
        >,
    > {
        loop {
            if let Some(byte) = poll_incoming(conn).map_err(WaitForStopReasonError::Connection)? {
                return Ok(Event::IncomingData(byte));
            }

            // Check if we hit a breakpoint before executing
            let eip = target.emu.regs().get_eip() as u32;
            if target.emu.bp.is_bp(eip as u64) && !target.single_step {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }

            // Single step mode - return after one instruction
            if target.single_step {
                target.single_step = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::DoneStep));
            }

            // Execute one instruction
            let result = target.emu.step();
            if !result {
                // Emulation ended
                return Ok(Event::TargetStopped(SingleThreadStopReason::Terminated(
                    Signal::SIGTERM,
                )));
            }

            // Check if a library was loaded during this step
            if target.emu.library_loaded {
                target.emu.library_loaded = false;
                return Ok(Event::TargetStopped(SingleThreadStopReason::Library(())));
            }

            // Check for memory watchpoints
            for mem_op in &target.emu.memory_operations {
                if target.emu.bp.is_bp_mem_read(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Read,
                        addr: mem_op.address as u32,
                    }));
                }
                if target.emu.bp.is_bp_mem_write_addr(mem_op.address) {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Watch {
                        tid: (),
                        kind: gdbstub::target::ext::breakpoints::WatchKind::Write,
                        addr: mem_op.address as u32,
                    }));
                }
            }

            // Check for breakpoint after step
            let eip = target.emu.regs().get_eip() as u32;
            if target.emu.bp.is_bp(eip as u64) {
                return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
            }
        }
    }

    fn on_interrupt(
        _target: &mut Self::Target,
    ) -> Result<Option<Self::StopReason>, <Self::Target as Target>::Error> {
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}
