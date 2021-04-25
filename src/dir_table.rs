use alloc::string::String;

#[repr(C)]
pub struct DirTable {
    pub items : &'static mut [DirItem],
}

#[repr(C)] // 32 字节
#[derive(Debug, Clone, Copy)]
pub struct DirItem {
    pub name : [u8;15],
    pub attr : Attribute,
    pub start_block : u64,
    pub length : u64,
}

impl DirItem {
    pub fn new_file(filename : &String, start_block:usize, length:usize)->Self {
        let mut name = [0;15];
        for (idx, c) in filename.as_bytes().iter().enumerate() {
            if idx >= 15 {
                break;
            }
            name[idx] = *c;
        }
        Self {
            name,
            attr:Attribute::File,
            start_block:start_block as u64,
            length: length as u64,
        }
    }

    pub fn empty(&self)->bool {
        self.attr == Attribute::Free
    }

    pub fn new_dir(dirname : &String, start_block:usize, length:usize)->Self {
        assert!(dirname.len() <= 15);
        let mut name = [0;15];
        for (idx, c) in dirname.as_bytes().iter().enumerate() {
            name[idx] = *c;
        }
        Self {
            name,
            attr:Attribute::Directory,
            start_block:start_block as u64,
            length: length as u64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Attribute {
    Free = 0,
    File = 1,
    Directory = 1 << 1,
}
