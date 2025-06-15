//! A stereo plate reverberator developed by Lance Putnam, ported to Rust by MindBuffer.

#![cfg_attr(not(test), no_std)]

pub use reverb::Reverb;

mod delay_line;
mod reverb;
