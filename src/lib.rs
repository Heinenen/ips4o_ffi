#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, unused)]
mod bindings;

pub fn sort_u32(v: &mut [u32]) {
    let range = v.as_mut_ptr_range();
    unsafe { bindings::sort_u32(range.start, range.end) }
}

pub fn sort_par_u32(v: &mut [u32]) {
    let range = v.as_mut_ptr_range();
    unsafe { bindings::sort_par_u32(range.start, range.end) }
}

#[cfg(test)]
mod tests {
    use crate::{bindings::ffi_test, sort_u32, sort_par_u32};

    #[test]
    fn test_ffi() {
        unsafe { ffi_test() };
    }

    #[test]
    fn test_sort_u32() {
        let mut v = vec![10u32, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let mut compare = v.clone();
        compare.reverse();
        sort_u32(&mut v);
        assert!(v == compare);
    }

    #[test]
    fn test_sort_par_u32() {
        let mut v = vec![10u32, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let mut compare = v.clone();
        compare.reverse();
        sort_par_u32(&mut v);
        assert!(v == compare);
    }
}