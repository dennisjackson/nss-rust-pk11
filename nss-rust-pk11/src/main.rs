use nss_sys::*; 
use std::ffi::CString;

fn main() {
    let mut buf = [0u8; 100]; 
    unsafe {
        let empty = CString::default();
        let flags = NSS_INIT_READONLY
            | NSS_INIT_NOCERTDB
            | NSS_INIT_NOMODDB
            | NSS_INIT_FORCEOPEN
            | NSS_INIT_OPTIMIZESPACE;
        let context = 
            NSS_InitContext(
                empty.as_ptr(),
                empty.as_ptr(),
                empty.as_ptr(),
                empty.as_ptr(),
                std::ptr::null_mut(),
                flags,
            );
        PK11_GenerateRandom(buf.as_mut_ptr(), 100);
    }
    println!("{buf:?}"); 
}
