#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all)]

mod app;
mod style;
mod tool_card;
mod tools;
pub use app::App;
