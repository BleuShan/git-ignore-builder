#![forbid(future_incompatible)]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(result_flattening)]

#[cfg(test)]
use rstest_reuse;

pub mod config;
pub mod constants;
pub mod prelude;
