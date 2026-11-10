#[macro_use]
pub mod macros;
pub use mwemu_arch::utils::bits;
pub mod colors;
pub mod helpers;

pub use helpers::color_enabled;
pub use helpers::{disable_color, enable_color, filename_no_ext};
