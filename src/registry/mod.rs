mod content;

#[cfg(feature = "crates_io")]
mod crates_io;

#[cfg(feature = "crates_io")]
pub use crates_io::*;
