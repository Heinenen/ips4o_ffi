use crate::bindings::{ffi_test};

#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, unused)]
mod bindings;

fn main() {
    let mut v = vec![10u32, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    sort_u32(&mut v);
    unsafe {
        ffi_test();
    }
    println!("{v:?}");
}

fn sort_u32(v: &mut [u32]) {
    let range = v.as_mut_ptr_range();
    unsafe { bindings::sort_u32(range.start, range.end) }
}