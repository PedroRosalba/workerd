// A callback is called through a plain function pointer, so it cannot capture state.
pub fn t() -> jsg::RawFunctionCallback {
    let name = String::from("captured");
    jsg::function_callback(move |lock, args| {
        //~^ ERROR E0277
        let value = lock.new_object();
        value.set(lock, &name, args.this());
    })
}
