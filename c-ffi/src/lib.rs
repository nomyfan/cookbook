mod ffi;

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { ffi::add(a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sum = add(2, 2);
        assert_eq!(sum, 4);
    }
}
