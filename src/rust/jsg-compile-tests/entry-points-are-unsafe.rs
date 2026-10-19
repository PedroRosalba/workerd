// The entry point trusts its caller about the isolate's state, so it is unsafe.
use jsg::v8;
pub fn storage(isolate: *mut v8::ffi::Isolate) {
    let _storage = jsg::CallbackScope::new(isolate);
    //~^ ERROR E0133
}
