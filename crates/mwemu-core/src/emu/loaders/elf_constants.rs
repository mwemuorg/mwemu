pub const PT_LOAD: u32 = 1;
pub const ELF_PAGE_SIZE: u64 = 4096;
pub const ELF_PAGE_MASK: u64 = ELF_PAGE_SIZE - 1;
pub const ELF32_DYN_BASE: u64 = 0x56555000;
pub const ELF64_DYN_BASE: u64 = 0x555555554000;
pub const ELF64_STA_BASE: u64 = 0x400000;
pub const LIBC_BASE: u64 = 0x7ffff7da7000;
pub const LD_BASE: u64 = 0x7ffff7fd2000;
