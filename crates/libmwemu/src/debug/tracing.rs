// tracing.rs

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use crate::emu::Emu;

// Fixed-size trace record: 152 bytes per instruction
// This covers all general purpose registers + RIP + instruction counter
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TraceRecord {
    pub instruction_count: u64, // 8 bytes
    pub rip: u64,               // 8 bytes
    pub rflags: u64,            // 8 bytes
    pub rax: u64,               // 8 bytes
    pub rbx: u64,               // 8 bytes
    pub rcx: u64,               // 8 bytes
    pub rdx: u64,               // 8 bytes
    pub rsi: u64,               // 8 bytes
    pub rdi: u64,               // 8 bytes
    pub rbp: u64,               // 8 bytes
    pub rsp: u64,               // 8 bytes
    pub r8: u64,                // 8 bytes
    pub r9: u64,                // 8 bytes
    pub r10: u64,               // 8 bytes
    pub r11: u64,               // 8 bytes
    pub r12: u64,               // 8 bytes
    pub r13: u64,               // 8 bytes
    pub r14: u64,               // 8 bytes
    pub r15: u64,               // 8 bytes
                                // Total: 152 bytes
}

impl TraceRecord {
    pub fn capture(emu: &Emu, instruction_count: u64) -> Self {
        if emu.cfg.arch.is_aarch64() {
            let regs = emu.regs_aarch64();
            let nzcv = &regs.nzcv;
            let nzcv_val = ((nzcv.n as u64) << 31)
                | ((nzcv.z as u64) << 30)
                | ((nzcv.c as u64) << 29)
                | ((nzcv.v as u64) << 28);
            Self {
                instruction_count,
                rip: regs.pc,
                rflags: nzcv_val,
                rax: regs.x[0],
                rbx: regs.x[1],
                rcx: regs.x[2],
                rdx: regs.x[3],
                rsi: regs.x[4],
                rdi: regs.x[5],
                rbp: regs.x[29], // fp
                rsp: regs.sp,
                r8: regs.x[8],
                r9: regs.x[9],
                r10: regs.x[10],
                r11: regs.x[11],
                r12: regs.x[12],
                r13: regs.x[13],
                r14: regs.x[14],
                r15: regs.x[15],
            }
        } else {
            Self {
                instruction_count,
                rip: emu.regs().rip,
                rflags: emu.flags_snapshot().dump() as u64,
                rax: emu.regs().rax,
                rbx: emu.regs().rbx,
                rcx: emu.regs().rcx,
                rdx: emu.regs().rdx,
                rsi: emu.regs().rsi,
                rdi: emu.regs().rdi,
                rbp: emu.regs().rbp,
                rsp: emu.regs().rsp,
                r8: emu.regs().r8,
                r9: emu.regs().r9,
                r10: emu.regs().r10,
                r11: emu.regs().r11,
                r12: emu.regs().r12,
                r13: emu.regs().r13,
                r14: emu.regs().r14,
                r15: emu.regs().r15,
            }
        }
    }

    /// Serialize the record to its fixed 152-byte little-endian-on-x86 layout.
    /// Returns an owned array (built field by field) so no `unsafe` byte cast is
    /// needed; the 19 register-sized fields map 1:1 to 8-byte little-endian slots.
    pub fn to_bytes(&self) -> [u8; std::mem::size_of::<Self>()] {
        let fields = [
            self.instruction_count,
            self.rip,
            self.rflags,
            self.rax,
            self.rbx,
            self.rcx,
            self.rdx,
            self.rsi,
            self.rdi,
            self.rbp,
            self.rsp,
            self.r8,
            self.r9,
            self.r10,
            self.r11,
            self.r12,
            self.r13,
            self.r14,
            self.r15,
        ];
        let mut out = [0u8; std::mem::size_of::<Self>()];
        for (i, f) in fields.iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&f.to_ne_bytes());
        }
        out
    }
}

// Thread-local trace state. `RefCell`/`Cell` give us interior mutability with
// safe, checked access — no `unsafe`. Tracing is off the hot path (it only does
// anything once a writer is installed), so the borrow/cell checks are free in
// practice.
thread_local! {
    // The actual writer - None if tracing is disabled
    static TRACE_WRITER: RefCell<Option<BufWriter<File>>> = const { RefCell::new(None) };

    // Counter for periodic flushing
    static TRACE_RECORDS_WRITTEN: Cell<u64> = const { Cell::new(0) };

    // Start time for IPS calculations
    static TRACE_START_TIME: Cell<Option<Instant>> = const { Cell::new(None) };

    // Last time we logged an IPS line, for rate limiting (None until first log)
    static LAST_IPS_LOG: Cell<Option<Instant>> = const { Cell::new(None) };
}

pub fn init_tracing(path: impl AsRef<Path>) -> std::io::Result<()> {
    let file = File::create(path)?;
    // 16MB buffer for maximum efficiency
    let writer = BufWriter::with_capacity(16 * 1024 * 1024, file);

    TRACE_WRITER.with(|w| *w.borrow_mut() = Some(writer));
    TRACE_START_TIME.with(|t| t.set(Some(Instant::now())));
    LAST_IPS_LOG.with(|l| l.set(Some(Instant::now())));

    log::info!("📝 Trace logging initialized");
    Ok(())
}

#[inline(always)]
pub fn trace_instruction(emu: &Emu, instruction_count: u64) {
    TRACE_WRITER.with(|writer_cell| {
        let mut writer = writer_cell.borrow_mut();
        let Some(w) = writer.as_mut() else { return };

        // Capture into a stack-local record and write it out.
        let record = TraceRecord::capture(emu, instruction_count);
        if let Err(e) = w.write_all(&record.to_bytes()) {
            log::error!("Failed to write trace record: {}", e);
            return;
        }

        // Update counter and flush periodically
        let count = TRACE_RECORDS_WRITTEN.with(|c| {
            let n = c.get() + 1;
            c.set(n);
            n
        });

        // Calculate and log IPS every 10M instructions
        if count % 10_000_000 == 0 {
            if let Some(start_time) = TRACE_START_TIME.with(|s| s.get()) {
                let elapsed = start_time.elapsed();
                let elapsed_secs = elapsed.as_secs_f64();
                if elapsed_secs > 0.0 {
                    let ips = instruction_count as f64 / elapsed_secs;

                    // Rate limit IPS logs to once per second
                    let now = Instant::now();
                    let should_log = LAST_IPS_LOG
                        .with(|l| l.get())
                        .is_none_or(|last| now.duration_since(last).as_secs() >= 1);
                    if should_log {
                        log::info!(
                            "⚡ IPS: {:.2} ({} instructions in {:.2}s)",
                            ips,
                            instruction_count,
                            elapsed_secs
                        );
                        LAST_IPS_LOG.with(|l| l.set(Some(now)));
                    }
                }
            }
        }

        // Flush every 1M records to avoid losing too much data if we crash
        if count % 1_000_000 == 0 {
            if let Err(e) = w.flush() {
                log::error!("Failed to flush trace: {}", e);
            } else {
                log::debug!("Trace: Flushed {} records", count);
            }
        }
    });
}

pub fn flush_trace() {
    TRACE_WRITER.with(|writer_cell| {
        let mut writer = writer_cell.borrow_mut();
        if let Some(w) = writer.as_mut() {
            if let Err(e) = w.flush() {
                log::error!("Failed to flush trace: {}", e);
            } else {
                let count = TRACE_RECORDS_WRITTEN.with(|c| c.get());
                log::info!("📝 Flushed {} trace records", count);
            }
        }
    });
}
