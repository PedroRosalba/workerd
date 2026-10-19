// The arguments of a callback live in the callback's scope and cannot leave it.
use jsg::v8;
pub fn t() -> jsg::RawFunctionCallback {
    jsg::function_callback(|_lock, args| {
        let _this: v8::Local<'static, v8::Value> = args.this();
        //~^ ERROR lifetime may not live long enough
    })
}
