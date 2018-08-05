#![no_std]
#![cfg_attr(feature = "no-panic", feature(use_extern_macros))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        cast_lossless,
        cyclomatic_complexity,
        many_single_char_names,
        needless_pass_by_value,
        unreadable_literal,
    )
)]

#[cfg(feature = "no-panic")]
extern crate no_panic;

mod buffer;
mod common;
mod d2s;
#[cfg(not(feature = "small"))]
mod d2s_full_table;
#[cfg(feature = "small")]
mod d2s_small_table;
mod digit_table;
mod f2s;
#[cfg(not(integer128))]
mod mulshift128;
mod pretty;

pub use buffer::{Buffer, Float};

/// Unsafe functions that exactly mirror the API of the C implementation of Ryū.
pub mod raw {
    pub use d2s::d2s_buffered_n;
    pub use f2s::f2s_buffered_n;
}
