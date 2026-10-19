// Boxing a handle does not let it leave its scope.
use jsg::v8;
pub fn t(isolate: *mut v8::ffi::Isolate) -> Box<v8::Local<'static, v8::Object>> {
    // SAFETY: only type-checked.
    jsg::callback_scope!(unsafe let lock, isolate);
    Box::new(lock.new_object())
    //~^ ERROR E0515
    //~| ERROR E0515
}
