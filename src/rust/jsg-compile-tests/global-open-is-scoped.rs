// Opening a `Global` needs a lock, and the handle it returns is tied to that lock's scope.
use jsg::v8;
pub fn t(
    isolate: *mut v8::ffi::Isolate,
    global: &v8::Global<v8::Object>,
) -> v8::Local<'static, v8::Object> {
    // SAFETY: only type-checked.
    jsg::callback_scope!(unsafe let lock, isolate);
    global.as_local(lock)
    //~^ ERROR E0515
    //~| ERROR E0515
}
