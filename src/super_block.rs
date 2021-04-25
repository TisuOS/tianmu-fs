//! # 天目文件系统格式
//! 文件系统使用以节点（node）为单位存储文件，为链式结构
//!
//! 2021年4月13日 zg

#![allow(dead_code)]

pub const MAGIC : [u8;4] = [2,3,3,3];

#[derive(Clone, Copy, Debug)]
pub struct SuperBlock {
    pub jump1   : u8,
    pub jump2   : u8,
    pub jump3   : u8,
    pub oem     : [u8;8],
    pub setting : Setting,
}

impl SuperBlock {
    pub fn new(block_size: usize, block_num: usize, map_offset: usize, root_offset: usize)->Self {
        Self {
            jump1: 0,
            jump2: 0,
            jump3: 0,
            oem: [0;8],
            setting: Setting{
                bytes_per_block: block_size as u64,
                block_num: block_num as u64,
                block_map_offset: map_offset as u64,
                root_table_offset: root_offset as u64,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Setting {
    pub bytes_per_block : u64,
    pub block_num : u64,
    pub block_map_offset : u64,
    pub root_table_offset : u64,
}


