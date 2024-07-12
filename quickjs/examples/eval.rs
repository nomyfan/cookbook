use std::ffi::{CStr, CString};

mod quickjs {
    #![allow(warnings, unused)]

    include!("../quickjs-sys/bindings.rs");
}

use quickjs::{
    JSContext, JSRuntime, JSValue, JS_Eval, JS_FreeCString, JS_FreeContext, JS_FreeRuntime,
    JS_FreeValue__extern, JS_NewContext, JS_NewRuntime, JS_ToInt32,
};

#[derive(Debug)]
#[allow(dead_code)]
enum Value {
    Int(i32),
    String(String),
    // ...
}

impl From<(*mut JSContext, JSValue)> for Value {
    fn from((ctx, jsvalue): (*mut JSContext, JSValue)) -> Self {
        let value = match jsvalue.tag as i32 {
            quickjs::JS_TAG_INT => {
                let ret = 0i32;
                let ptr = &ret as *const i32 as *mut i32;
                unsafe { JS_ToInt32(ctx, ptr, jsvalue) };

                Value::Int(ret)
            }
            quickjs::JS_TAG_STRING => {
                let str_ptr = unsafe { quickjs::JS_ToCString__extern(ctx, jsvalue) };
                let str_value = unsafe { CStr::from_ptr(str_ptr).to_string_lossy().to_string() };
                unsafe {
                    JS_FreeCString(ctx, str_ptr);
                }
                Value::String(str_value)
            }
            _ => unimplemented!(),
        };

        unsafe {
            JS_FreeValue__extern(ctx, jsvalue);
        }

        value
    }
}

struct Engine {
    rt: *mut JSRuntime,
    ctx: *mut JSContext,
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            JS_FreeContext(self.ctx);
            JS_FreeRuntime(self.rt);
        }
    }
}

impl Engine {
    fn eval(&self, code: &str) -> JSValue {
        let bytes = code.as_bytes();
        let input_size = bytes.len() - 1;
        let input = CStr::from_bytes_with_nul(bytes).unwrap();
        let filename = CStr::from_bytes_with_nul("script\0".as_bytes()).unwrap();

        unsafe {
            JS_Eval(
                self.ctx,
                input.as_ptr(),
                input_size,
                filename.as_ptr(),
                quickjs::JS_EVAL_TYPE_GLOBAL as i32,
            )
        }
    }

    fn set_global_string(&mut self, name: &str, value: &str) {
        let global = unsafe { quickjs::JS_GetGlobalObject(self.ctx) };

        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();

        let jsvalue = unsafe { quickjs::JS_NewString(self.ctx, value.as_ptr()) };
        unsafe {
            let ret = quickjs::JS_SetPropertyStr(self.ctx, global, name.as_ptr(), jsvalue);
            JS_FreeValue__extern(self.ctx, global);

            if ret == -1 {
                panic!("Failed to set global string");
            }
        };
    }

    fn remove_global_property(&mut self, name: &str) {
        let global = unsafe { quickjs::JS_GetGlobalObject(self.ctx) };

        let name = CString::new(name).unwrap();
        unsafe {
            quickjs::JS_DeleteProperty(
                self.ctx,
                global,
                quickjs::JS_NewAtom(self.ctx, name.as_ptr()),
                quickjs::JS_PROP_THROW as i32,
            );
            JS_FreeValue__extern(self.ctx, global);
        }
    }
}

fn main() {
    let (rt, ctx) = unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);

        (rt, ctx)
    };

    let mut engine = Engine { rt, ctx };

    let value: Value = (engine.ctx, engine.eval("1+1\0")).into();
    println!("{:?}", value);

    engine.set_global_string("foo", "bar");
    let value: Value = (engine.ctx, engine.eval("`Hello, ${foo}!`\0")).into();
    engine.remove_global_property("foo");
    println!("{:?}", value);
}
