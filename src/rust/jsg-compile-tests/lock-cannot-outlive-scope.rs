// A lock cannot outlive the storage of its scope.
use jsg::v8;
pub fn t(isolate: *mut v8::ffi::Isolate) {
    let escaped;
    {
        // SAFETY: only type-checked.
        jsg::callback_scope!(unsafe let lock, isolate);
        //~^ ERROR E0716
        //~| ERROR E0716
        escaped = lock;
    }
    let _ = escaped.new_object();
}
