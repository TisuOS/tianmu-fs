//! # 天目文件系统
//! 
//! 2021年4月14日 zg
#![no_std]
extern crate alloc;
mod super_block;
mod require;
mod tisu_impl;
mod block_map;
mod dir_table;

pub use block_map::*;
pub use dir_table::*;
pub use super_block::*;

pub const MAGIC : u64 = 0x2333;
pub const END : u64 = !0;