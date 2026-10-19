// A function callback reads its arguments and sets its return value in its own scope, without
// `unsafe`.
pub fn t() -> jsg::RawFunctionCallback {
    jsg::function_callback(|lock, args| {
        let object = lock.new_object();
        object.set(lock, "this", args.this());
        args.set_return_value(object.into());
    })
}
