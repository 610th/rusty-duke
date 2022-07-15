//! # Rusty Duke
//!
//! Unofficial implementation of the [The Duke](https://boardgamegeek.com/boardgame/36235/duke) boardgame.
//!
//! Currently supports player vs. AI and AI vs AI.
//!
//! ## Try it
//! `cargo run --release`

#[macro_use]
extern crate lazy_static;

pub mod ai;
pub mod logic;
