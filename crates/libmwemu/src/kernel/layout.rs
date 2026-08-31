//! Where the emulated kernel address space lives.
//!
//! The numbers mirror the real layouts so a dumped pointer looks like what an
//! analyst expects from a crash log, and — more importantly — so that the
//! distances between regions stay inside the ranges the relocations can encode.
//! On x86_64 a module is built with `-mcmodel=kernel`: calls out to the kernel
//! are `R_X86_64_PLT32`, a signed 32-bit displacement, so the module image and
//! the kernel stub area *must* sit within ±2GB of each other. Heap and vmalloc
//! addresses only ever travel in registers, so they are free to live far away.

use crate::kernel::KernelOs;

/// One kernel's address-space plan.
#[derive(Debug, Clone, Copy)]
pub struct KernelLayout {
    /// Base of the synthetic "kernel text": one stub slot per imported symbol.
    pub stub_base: u64,
    /// Bytes reserved for stubs (`STUB_SLOT` bytes each).
    pub stub_size: u64,
    /// Base of the synthetic kernel data (exported variables like `jiffies`).
    pub data_base: u64,
    /// Bytes reserved for kernel data.
    pub data_size: u64,
    /// Where a loaded driver image is placed.
    pub module_base: u64,
    /// Base of the slab / pool allocator region.
    pub heap_base: u64,
    /// Bytes the slab region may hand out before it is exhausted.
    pub heap_size: u64,
    /// Base of the vmalloc / non-paged large allocation region.
    pub vmalloc_base: u64,
    /// Bytes the vmalloc region may hand out.
    pub vmalloc_size: u64,
    /// Base of the emulated kernel stack.
    pub stack_base: u64,
    /// Kernel stack size.
    pub stack_size: u64,
}

impl KernelLayout {
    /// One page just past the stub area, used as the return address when the
    /// emulator calls into the driver. It is deliberately *outside* the stub
    /// range so returning there is an ordinary branch that ends the run, not
    /// another intercepted kernel call.
    pub fn retpad(&self) -> u64 {
        self.stub_base + self.stub_size
    }
}

/// Bytes reserved per imported symbol in the stub area. Each slot holds a
/// single `ret` so a stub that is somehow executed instead of intercepted
/// still returns to its caller rather than running into the next symbol.
pub const STUB_SLOT: u64 = 16;

/// Gap left between two consecutive heap chunks. A linear overflow off the end
/// of a chunk lands in this unmapped hole and faults immediately, which is what
/// turns "silent corruption" into a reported finding.
pub const HEAP_REDZONE: u64 = 0x1000;

impl KernelLayout {
    pub fn for_os(os: KernelOs) -> KernelLayout {
        match os {
            // Linux x86_64: kernel text at 0xffffffff81000000, modules at
            // 0xffffffffc0000000 (2GB apart at most), direct map at
            // 0xffff888000000000, vmalloc at 0xffffc90000000000.
            KernelOs::Linux => KernelLayout {
                stub_base: 0xffffffff81000000,
                stub_size: 0x100000,
                data_base: 0xffffffff82000000,
                data_size: 0x100000,
                module_base: 0xffffffffc0000000,
                heap_base: 0xffff888000000000,
                heap_size: 0x10000000,
                vmalloc_base: 0xffffc90000000000,
                vmalloc_size: 0x10000000,
                stack_base: 0xffffc90000100000,
                stack_size: 0x20000,
            },
            // Windows x64: ntoskrnl around 0xfffff80000000000, drivers just
            // above it, non-paged pool in the 0xffffe000... range.
            KernelOs::Windows => KernelLayout {
                stub_base: 0xfffff80000000000,
                stub_size: 0x100000,
                data_base: 0xfffff80000200000,
                data_size: 0x100000,
                module_base: 0xfffff80000400000,
                heap_base: 0xffffe00000000000,
                heap_size: 0x10000000,
                vmalloc_base: 0xffffe10000000000,
                vmalloc_size: 0x10000000,
                stack_base: 0xffffe20000000000,
                stack_size: 0x20000,
            },
            // XNU: kernel text in the 0xffffff80... range, kexts right after.
            KernelOs::MacOS => KernelLayout {
                stub_base: 0xffffff8000200000,
                stub_size: 0x100000,
                data_base: 0xffffff8000400000,
                data_size: 0x100000,
                module_base: 0xffffff8000600000,
                heap_base: 0xffffff9000000000,
                heap_size: 0x10000000,
                vmalloc_base: 0xffffff9100000000,
                vmalloc_size: 0x10000000,
                stack_base: 0xffffff9200000000,
                stack_size: 0x20000,
            },
        }
    }

    /// True when `addr` falls inside the stub area, i.e. the driver is calling
    /// out to the kernel rather than into its own image.
    pub fn is_stub(&self, addr: u64) -> bool {
        addr >= self.stub_base && addr < self.stub_base + self.stub_size
    }
}
