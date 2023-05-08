use crate::bindings::{hello};

mod bindings;

fn main() {
    unsafe { hello() }
    println!("Hello, world!");
}
