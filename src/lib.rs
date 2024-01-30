#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod sys;


pub fn malloc<T>(size: usize, align: i32) -> *mut T {
    unsafe { sys::MKL_malloc(size, align) as *mut T }
}

pub fn free<T>(ptr: *mut T) {
    unsafe { sys::MKL_free(ptr as *mut ::std::os::raw::c_void) };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc() {
        let ptr: *mut i32 = malloc(8, 64);
        assert!(!ptr.is_null());
        free(ptr);
    }

    #[test]
    fn test_malloc_2_ptrs() {
        let ptr1: *mut i32 = malloc(8, 64);
        assert!(!ptr1.is_null());
        
        let ptr2: *mut i32 = malloc(8, 64);
        assert!(!ptr2.is_null());

        assert_ne!(ptr1, ptr2);

        free(ptr1);
        free(ptr2);
    }

    #[test]
    fn test_free() {
        let ptr1: *mut i32 = malloc(8, 64);
        assert!(!ptr1.is_null());

        free(ptr1);
        
        let ptr2: *mut i32 = malloc(8, 64);
        assert!(!ptr2.is_null());

        assert_eq!(ptr1, ptr2);
        
        free(ptr2);
    }
}
