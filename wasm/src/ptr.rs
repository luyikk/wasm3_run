use std::ops::Deref;

#[repr(C)]
#[derive(Debug,Copy, Clone)]
pub struct WMemory {
    pub address:u32,
    pub len:u32
}

impl WMemory {
    pub fn new(address:u32,len:u32)->Self{
        Self{
            address,
            len
        }
    }

    pub fn get_ptr(&self,base_address:* const u8)->* const [u8] {
        let ptr = self.address as usize + base_address as usize;
        std::ptr::slice_from_raw_parts(ptr as *const u8, self.len as usize)
    }

    pub fn get_mut_ptr(&self,base_address:* mut u8)->* mut [u8] {
        let ptr = self.address as usize + base_address as usize;
        std::ptr::slice_from_raw_parts_mut(ptr as *mut u8, self.len as usize)
    }

    pub fn get_str(&self, base_address: *const u8) ->&str {
        let ptr = self.address as usize + base_address as usize;
        unsafe {
            std::str::from_utf8_unchecked(&*std::ptr::slice_from_raw_parts(ptr as *const u8, self.len as usize))
        }
    }
}

impl From<u64> for WMemory {
    fn from(value: u64) -> Self {
        unsafe{
            (&value as *const u64 as * const Self).read()
        }
    }
}

impl Into<u64> for WMemory {
    fn into(self) -> u64 {
       unsafe{
           (&self as *const Self as *const u64 ).read()
       }
    }
}


impl Deref for WMemory {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self as *const Self as *const u64)
        }
    }
}