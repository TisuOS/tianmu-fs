
#[repr(C)]
pub struct BlockMapItem {
    pub flag : u64,
}

impl BlockMapItem {
    pub fn next(&self)->Option<u64> {
        if self.flag == 0 { None }
        else { Some(self.flag) }
    }
}