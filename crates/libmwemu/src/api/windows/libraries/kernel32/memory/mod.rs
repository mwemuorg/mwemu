mod flush_instruction_cache;
pub use flush_instruction_cache::FlushInstructionCache;

mod global_alloc;
pub use global_alloc::GlobalAlloc;

mod global_free;
pub use global_free::GlobalFree;

mod heap_alloc;
pub use heap_alloc::HeapAlloc;

mod heap_create;
pub use heap_create::HeapCreate;

mod heap_destroy;
pub use heap_destroy::HeapDestroy;

mod heap_free;
pub use heap_free::HeapFree;

mod heap_re_alloc;
pub use heap_re_alloc::HeapReAlloc;

mod heap_size;
pub use heap_size::HeapSize;

mod local_alloc;
pub use local_alloc::LocalAlloc;

mod local_free;
pub use local_free::LocalFree;

mod map_view_of_file;
pub use map_view_of_file::MapViewOfFile;

mod read_process_memory;
pub use read_process_memory::ReadProcessMemory;

mod virtual_alloc;
pub use virtual_alloc::VirtualAlloc;

mod virtual_alloc_ex;
pub use virtual_alloc_ex::VirtualAllocEx;

mod virtual_alloc_ex_numa;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;

mod virtual_free;
pub use virtual_free::VirtualFree;

mod virtual_protect;
pub use virtual_protect::VirtualProtect;

mod virtual_protect_ex;
pub use virtual_protect_ex::VirtualProtectEx;

mod write_process_memory;
pub use write_process_memory::WriteProcessMemory;

mod pointer_probe;
pub use pointer_probe::{
    IsBadCodePtr, IsBadHugeReadPtr, IsBadHugeWritePtr, IsBadReadPtr, IsBadWritePtr,
};

mod allocation_handle;
pub use allocation_handle::{GlobalHandle, LocalHandle};
