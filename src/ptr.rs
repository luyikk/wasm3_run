#[repr(C)]
#[derive(Debug)]
pub struct WPtr{
    pub address:u32,
    pub len:u32
}

impl WPtr {
    pub fn new(address:u32,len:u32)->Self{
        Self{
            address,
            len
        }
    }
    pub fn from_u64(value:u64 )->Self{
        unsafe{
            (&value as *const u64 as * const Self).read()
        }
    }

    pub fn into_u64(self)->u64 {
        unsafe {
            (&self as *const Self as *const u64 ).read()
        }
    }
}