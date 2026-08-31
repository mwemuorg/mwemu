//! Memory-safety verdicts for emulated kernel code.
//!
//! The allocator ledger in [`crate::kernel::heap`] knows which chunks are live
//! and which are in quarantine; this module is what asks that question on every
//! memory access the driver makes and turns a bad answer into a finding.
//!
//! It is deliberately shaped like KASAN: a freed chunk stays mapped and
//! poisoned, so execution continues after the first stale dereference and one
//! run can surface the whole chain (the read, the write, the indirect call)
//! instead of stopping at the first symptom.

use crate::emu::Emu;
use crate::kernel::heap::KernelChunk;
use crate::utils::helpers::unlikely;

/// SLUB's `POISON_FREE`: every byte of a quarantined chunk reads back as `0x6b`.
const POISON_FREE: u8 = 0x6b;

/// True when `addr` came from free poison — it is not an address at all, but
/// bytes read out of a quarantined object and used as one.
///
/// The comparison is per page, not exact, because such a pointer is almost
/// never dereferenced bare: the code that loaded it goes on to read a field, so
/// what actually reaches memory is poison plus a small struct offset. Both
/// pointer widths are covered: a 64-bit field yields all eight poison bytes, a
/// 32-bit one the low four with a zero upper half.
pub fn is_free_poison(addr: u64) -> bool {
    const POISON64: u64 = u64::from_ne_bytes([POISON_FREE; 8]);
    const POISON32: u64 = 0x6b6b_6b6b;
    const PAGE: u64 = !0xfff;
    (addr & PAGE) == (POISON64 & PAGE) || (addr & PAGE) == (POISON32 & PAGE)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    /// Read from a chunk that has already been freed.
    UseAfterFree,
    /// Write into a chunk that has already been freed.
    WriteAfterFree,
    /// Execution transferred to an address read out of a freed chunk.
    FreedFunctionPointerCall,
    /// A pointer loaded out of a freed object was dereferenced. The address
    /// itself is the slab free poison, so it is not merely a wild pointer —
    /// its provenance is certain.
    PoisonDereference,
    /// `kfree()` on a pointer that is already in quarantine.
    DoubleFree,
    /// `kfree()` on something that is not the base of a known chunk.
    InvalidFree,
    /// Access past the requested size but still inside the slab bucket.
    HeapOverflow,
    /// Still allocated when the module's exit path finished.
    Leak,
}

impl FindingKind {
    pub fn label(self) -> &'static str {
        match self {
            FindingKind::UseAfterFree => "use-after-free (read)",
            FindingKind::WriteAfterFree => "use-after-free (write)",
            FindingKind::FreedFunctionPointerCall => "use-after-free (indirect call)",
            FindingKind::PoisonDereference => "use-after-free (poisoned pointer dereference)",
            FindingKind::DoubleFree => "double-free",
            FindingKind::InvalidFree => "invalid-free",
            FindingKind::HeapOverflow => "slab-out-of-bounds",
            FindingKind::Leak => "memory-leak",
        }
    }

    /// Short machine-friendly tag, used by the MCP surface and tests.
    pub fn tag(self) -> &'static str {
        match self {
            FindingKind::UseAfterFree => "use_after_free_read",
            FindingKind::WriteAfterFree => "use_after_free_write",
            FindingKind::FreedFunctionPointerCall => "use_after_free_call",
            FindingKind::PoisonDereference => "use_after_free_poison_deref",
            FindingKind::DoubleFree => "double_free",
            FindingKind::InvalidFree => "invalid_free",
            FindingKind::HeapOverflow => "slab_out_of_bounds",
            FindingKind::Leak => "memory_leak",
        }
    }

    pub fn is_use_after_free(self) -> bool {
        matches!(
            self,
            FindingKind::UseAfterFree
                | FindingKind::WriteAfterFree
                | FindingKind::FreedFunctionPointerCall
                | FindingKind::PoisonDereference
        )
    }
}

/// Snapshot of a chunk's provenance, copied into the finding so the report
/// survives independently of the ledger.
#[derive(Debug, Clone, Default)]
pub struct ChunkOrigin {
    pub addr: u64,
    pub size: u64,
    pub req_size: u64,
    pub cache: String,
    pub alloc_api: String,
    pub alloc_pos: u64,
    pub alloc_rip: u64,
    pub free_api: String,
    pub free_pos: u64,
    pub free_rip: u64,
}

impl From<&KernelChunk> for ChunkOrigin {
    fn from(c: &KernelChunk) -> ChunkOrigin {
        ChunkOrigin {
            addr: c.addr,
            size: c.size,
            req_size: c.req_size,
            cache: c.cache.clone(),
            alloc_api: c.alloc_api.clone(),
            alloc_pos: c.alloc_pos,
            alloc_rip: c.alloc_rip,
            free_api: c.free_api.clone(),
            free_pos: c.free_pos,
            free_rip: c.free_rip,
        }
    }
}

/// One memory-safety verdict.
#[derive(Debug, Clone)]
pub struct Finding {
    pub kind: FindingKind,
    /// Instruction counter when it was first observed.
    pub pos: u64,
    /// Instruction that performed the access.
    pub rip: u64,
    /// Address touched.
    pub addr: u64,
    /// Access width in bytes (0 for allocator-level findings).
    pub size: u32,
    pub origin: ChunkOrigin,
    /// How many times this exact (kind, rip, chunk) triple was seen.
    pub hits: u64,
}

impl Finding {
    /// KASAN-style report, one finding per block.
    pub fn report(&self) -> String {
        let o = &self.origin;
        let mut out = format!(
            "BUG: KMWEMU: {} in {} of size {} at addr 0x{:x}\n",
            self.kind.label(),
            o.cache,
            self.size,
            self.addr
        );
        if self.rip != 0 {
            out.push_str(&format!(
                "  faulting instruction at 0x{:x} (step {})\n",
                self.rip, self.pos
            ));
        }
        if o.addr != 0 {
            out.push_str(&format!(
                "  object 0x{:x}..0x{:x} (requested {} bytes, bucket {}), offset {}\n",
                o.addr,
                o.addr + o.size,
                o.req_size,
                o.size,
                self.addr.wrapping_sub(o.addr)
            ));
            out.push_str(&format!(
                "  allocated by {} at 0x{:x} (step {})\n",
                o.alloc_api, o.alloc_rip, o.alloc_pos
            ));
            if !o.free_api.is_empty() {
                out.push_str(&format!(
                    "  freed by {} at 0x{:x} (step {})\n",
                    o.free_api, o.free_rip, o.free_pos
                ));
            }
        }
        if self.hits > 1 {
            out.push_str(&format!("  seen {} times\n", self.hits));
        }
        out
    }
}

impl Emu {
    /// Verdict for one memory access made by emulated kernel code.
    ///
    /// Called from the operand read/write paths behind the `kernel_guard`
    /// flag, so it costs a predictable branch when no driver is loaded.
    pub fn kernel_guard_access(&mut self, rip: u64, addr: u64, bytes: u32, is_write: bool) {
        let Some(kernel) = self.kernel.as_ref() else {
            return;
        };

        // The address itself is slab free poison: the pointer was loaded out of
        // a quarantined object, so this is a use-after-free even though the
        // address it produced belongs to no chunk at all.
        if unlikely(is_free_poison(addr)) {
            let origin = ChunkOrigin {
                cache: "freed object".to_string(),
                ..Default::default()
            };
            self.kernel_report(FindingKind::PoisonDereference, rip, addr, bytes, origin);
            return;
        }

        let Some(idx) = kernel.heap.index_of(addr) else {
            return;
        };
        let chunk = kernel.heap.get(idx);

        let kind = if chunk.is_freed() {
            if is_write {
                FindingKind::WriteAfterFree
            } else {
                FindingKind::UseAfterFree
            }
        } else if chunk.offset_of(addr) >= chunk.req_size {
            FindingKind::HeapOverflow
        } else {
            return;
        };

        let origin = ChunkOrigin::from(chunk);
        self.kernel_report(kind, rip, addr, bytes, origin);
    }

    /// Record a finding, collapsing repeats of the same (kind, rip, object).
    pub fn kernel_report(
        &mut self,
        kind: FindingKind,
        rip: u64,
        addr: u64,
        bytes: u32,
        origin: ChunkOrigin,
    ) {
        let pos = self.pos;
        let Some(kernel) = self.kernel.as_mut() else {
            return;
        };

        if let Some(existing) = kernel
            .findings
            .iter_mut()
            .find(|f| f.kind == kind && f.rip == rip && f.origin.addr == origin.addr)
        {
            existing.hits += 1;
            return;
        }

        let finding = Finding {
            kind,
            pos,
            rip,
            addr,
            size: bytes,
            origin,
            hits: 1,
        };
        log::warn!("{}", finding.report().trim_end());
        kernel.findings.push(finding);
    }

    /// Findings collected so far, most severe first is not enforced here —
    /// they stay in discovery order so the sequence tells the story.
    pub fn kernel_findings(&self) -> &[Finding] {
        match self.kernel.as_ref() {
            Some(k) => &k.findings,
            None => &[],
        }
    }

    /// True when at least one use-after-free was observed.
    pub fn kernel_found_uaf(&self) -> bool {
        self.kernel_findings()
            .iter()
            .any(|f| f.kind.is_use_after_free())
    }

    /// Turn every chunk still live into a leak finding. Call after the module's
    /// exit path has run, when nothing should be outstanding any more.
    pub fn kernel_check_leaks(&mut self) {
        let leaked: Vec<ChunkOrigin> = match self.kernel.as_ref() {
            Some(k) => k.heap.live().map(ChunkOrigin::from).collect(),
            None => return,
        };
        for origin in leaked {
            let rip = origin.alloc_rip;
            let addr = origin.addr;
            self.kernel_report(FindingKind::Leak, rip, addr, 0, origin);
        }
    }
}
