// A panic inside a scope propagates out of it.
use jsg::v8;
pub fn t(isolate: *mut v8::ffi::Isolate) {
    let result = std::panic::catch_unwind(|| {
        // SAFETY: only type-checked.
        jsg::callback_scope!(unsafe let lock, isolate);
        let _ = lock.new_object();
        panic!("boom");
    });
    let _ = result;
}
