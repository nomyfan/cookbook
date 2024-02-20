mod ffi;

#[cfg(test)]
mod tests {
    use crate::ffi::crc32;
    use std::os::raw::c_uint;

    #[test]
    fn it_works() {
        let s = "hello";
        unsafe {
            assert_eq!(crc32(0, s.as_ptr(), s.len() as c_uint), 0x3610a686);
        }
    }
}
