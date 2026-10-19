// JavaScript can call back into a resource while one of its methods runs, so a method must not
// get exclusive access to the resource.

use jsg_macros::jsg_method;
use jsg_macros::jsg_resource;

#[jsg_resource]
pub struct Counter {
    count: u32,
}

#[jsg_resource]
impl Counter {
    #[jsg_method]
    pub fn increment(&mut self) {
        //~^ ERROR a resource method takes `&self`
        self.count += 1;
    }

    #[jsg_method]
    pub fn consume(self) {} //~ ERROR a resource method takes `&self`
}
