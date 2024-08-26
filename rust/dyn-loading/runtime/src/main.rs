use libloading::{Library, Symbol};

// run via: cargo run $PWD/../plugin-a/target/release/libplugin_a.dylib
fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let path = argv.get(1).unwrap();
    unsafe {
        let lib = Library::new(path).unwrap();

        // ABI stability between plugins and the runtime needs to take into consideration.
        let version: Symbol<*const i32> = lib.get(b"PLUGIN_VERSION").unwrap();
        let hook_a: Symbol<unsafe extern "C" fn(i32) -> i32> = lib.get(b"plugin_hook_a").unwrap();

        println!("Version = {}", version.as_ref().unwrap());
        println!("plugin_hook_a(1) = {}", hook_a(1));
    }
}
