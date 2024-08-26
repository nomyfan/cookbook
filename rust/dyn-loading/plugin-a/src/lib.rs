#[no_mangle]
pub fn plugin_hook_a(a: i32) -> i32 {
    rand::random::<i32>().wrapping_add(a)
}

#[no_mangle]
pub static PLUGIN_VERSION: i32 = 1;
