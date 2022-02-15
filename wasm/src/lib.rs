pub mod ptr;

use std::alloc::{alloc, dealloc, Layout};
use crate::ptr::WMemory;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C"  fn malloc(len:u32)->u64{
    unsafe {
        let ptr = alloc(Layout::from_size_align_unchecked(len as usize, 16)) as u32;
        WMemory::new(ptr, len).into()
    }
}

#[no_mangle]
pub unsafe extern "C"  fn free(ptr:u64){
    let ptr=WMemory::from(ptr);
    dealloc(ptr.address as * mut u8, Layout::from_size_align_unchecked(ptr.len as usize, 16))
}


#[link(wasm_import_module = "default")]
extern "C" {
    #[link_name = "logout"]
    fn logout(msg:&str);
}


#[no_mangle]
pub extern "C" fn print(ptr:u64) {
    unsafe {
        let ptr=WMemory::from(ptr);
        let str=ptr.get_str(0 as *const u8);
        logout(str);
    }
}

