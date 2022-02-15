
pub mod ptr;

use wasm3::Environment;
use wasm3::Module;
use ptr::WMemory;

fn main()->anyhow::Result<()> {
    let env = Environment::new()?;
    let rt = env
        .create_runtime(1024)?;

    let module = Module::parse(&env, &include_bytes!("../wasm/pkg/wasm_de_bg.wasm")[..])?;
    let mut module = rt.load_module(module)?;

    module
        .link_closure("default", "logout", |rt, (ptr,len):(u32,u32)|{
            println!("log:{}",  WMemory::new(ptr,len).get_str(rt.memory().cast()));
            Ok(())
        } )?;

    let alloc_func= module.find_function::<u32,u64>("malloc")?;
    let free_func= module.find_function::<u64,()>("free")?;

    let hello="hello world";

    // 分配一块内存
    let memory = WMemory::from(alloc_func.call(hello.as_bytes().len() as u32)?);
    println!("{:?}", memory);

    // 将数据复制到内存
    let mem_ptr= memory.get_mut_ptr(rt.memory_mut().cast()).cast::<u8>();
    unsafe {
        mem_ptr.copy_from(hello.as_ptr(), hello.len());
    }
    // 打印内存
    let seconds= module.find_function::<u64,()>("print")?;
    seconds.call(*memory)?;

    //释放内存
    free_func.call(*memory)?;

    Ok(())
}


