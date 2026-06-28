//! `regatta_core` — the pure, deterministic core of Regatta.
//!
//! Rules for everything in this crate (enforced by the quality-gates law pack):
//! - **No I/O, no Tauri, no OS access.** Pure functions and data only.
//! - **No ambient time or randomness** — inject a `Clock` / `Rng` instead, so tests
//!   are reproducible and proofs are replayable.
//! - **100% line + branch coverage.** Split modules small enough to make that real.
//!
//! The glue layer (`src-tauri/`) wires this core to PTYs, IPC, the filesystem, and the webview.
#![forbid(unsafe_code)]

pub mod approval;
pub mod attention;
pub mod backend;
pub mod board;
pub mod budget;
pub mod config;
pub mod cost;
pub mod git;
pub mod layout;
pub mod runtime;
pub mod slug;
pub mod stream;
pub mod transcript;
pub mod view;
pub mod worktree;
