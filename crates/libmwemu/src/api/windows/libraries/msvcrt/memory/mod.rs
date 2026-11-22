pub use super::strings::units::{memchr, memcmp};
mod _aligned_malloc;
pub use _aligned_malloc::_aligned_malloc;

mod _msize;
pub use _msize::_msize;

mod calloc;
pub use calloc::calloc;

mod memcpy;
pub use memcpy::memcpy;

mod memmove;
pub use memmove::memmove;

mod memset;
pub use memset::memset;

mod free;
pub use free::free;

mod malloc;
pub use malloc::malloc;

mod realloc;
pub use realloc::realloc;
