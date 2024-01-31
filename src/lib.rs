use std::{mem::size_of, os::raw::c_void, slice};


#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod sys;


pub use sys::{
    VML_ACCURACY_MASK,
    VML_FTZDAZ_MASK,
    VML_ERRMODE_MASK,
};

pub use sys::{
    VML_HA,
    VML_LA,
    VML_EP,
    VML_FTZDAZ_ON,
    VML_FTZDAZ_OFF,
    VML_FTZDAZ_CURRENT,
    VML_ERRMODE_IGNORE,
    VML_ERRMODE_NOERR,
    VML_ERRMODE_ERRNO,
    VML_ERRMODE_STDERR,
    VML_ERRMODE_EXCEPT,
    VML_ERRMODE_CALLBACK,
    VML_ERRMODE_DEFAULT,
};

pub use sys::VSLStreamStatePtr;

pub use sys::{
    VSL_BRNG_SHIFT,
    VSL_BRNG_INC,
    VSL_BRNG_MCG31,
    VSL_BRNG_MRG32K3A,
    VSL_BRNG_MCG59,
    VSL_BRNG_WH,
    VSL_BRNG_SOBOL,
    VSL_BRNG_NIEDERR,
    VSL_BRNG_MT19937,
    VSL_BRNG_MT2203,
    VSL_BRNG_IABSTRACT,
    VSL_BRNG_DABSTRACT,
    VSL_BRNG_SABSTRACT,
    VSL_BRNG_SFMT19937,
    VSL_BRNG_NONDETERM,
    VSL_BRNG_ARS5,
    VSL_BRNG_PHILOX4X32X10,
    VSL_BRNG_RDRAND,
    VSL_BRNG_NONDETERM_NRETRIES,
};

pub use sys::{
    VSL_STATUS_OK,
    VSL_ERROR_OK,
    VSL_ERROR_FEATURE_NOT_IMPLEMENTED,
    VSL_ERROR_UNKNOWN,
    VSL_ERROR_BADARGS,
    VSL_ERROR_MEM_FAILURE,
    VSL_ERROR_NULL_PTR,
    VSL_ERROR_CPU_NOT_SUPPORTED,
};

pub use sys::{
    VSL_RNG_ERROR_INVALID_BRNG_INDEX,
    VSL_RNG_ERROR_LEAPFROG_UNSUPPORTED,
    VSL_RNG_ERROR_SKIPAHEAD_UNSUPPORTED,
    VSL_RNG_ERROR_SKIPAHEADEX_UNSUPPORTED,
    VSL_RNG_ERROR_BRNGS_INCOMPATIBLE,
    VSL_RNG_ERROR_BAD_STREAM,
    VSL_RNG_ERROR_BRNG_TABLE_FULL,
    VSL_RNG_ERROR_BAD_STREAM_STATE_SIZE,
    VSL_RNG_ERROR_BAD_WORD_SIZE,
    VSL_RNG_ERROR_BAD_NSEEDS,
    VSL_RNG_ERROR_BAD_NBITS,
    VSL_RNG_ERROR_QRNG_PERIOD_ELAPSED,
    VSL_RNG_ERROR_LEAPFROG_NSTREAMS_TOO_BIG,
    VSL_RNG_ERROR_BRNG_NOT_SUPPORTED,
    VSL_RNG_ERROR_BAD_UPDATE,
    VSL_RNG_ERROR_NO_NUMBERS,
    VSL_RNG_ERROR_INVALID_ABSTRACT_STREAM,
    VSL_RNG_ERROR_NONDETERM_NOT_SUPPORTED,
    VSL_RNG_ERROR_NONDETERM_NRETRIES_EXCEEDED,
    VSL_RNG_ERROR_ARS5_NOT_SUPPORTED,
};

pub use sys::{
    VSL_RNG_METHOD_UNIFORM_STD,
    VSL_RNG_METHOD_UNIFORM_STD_ACCURATE,
};


pub fn malloc<T>(n: usize, align: i32) -> *mut T {
    let ptr = unsafe { sys::MKL_malloc(n * size_of::<T>(), align) } as *mut T;
    if ptr.is_null() { panic!("MKL failed to allocate memory"); }
    ptr
}

pub fn free<T>(ptr: *const T) {
    unsafe { sys::MKL_free(ptr as *mut c_void) };
}

pub fn free_buffers() {
    unsafe { sys::MKL_Free_Buffers() };
}


pub fn vml_get_mode() -> u32 {
    unsafe { sys::vmlGetMode() }
}

pub fn vml_set_mode(new_mode: u32) -> u32 {
    unsafe { sys::vmlSetMode(new_mode) }
}


pub fn vsl_new_stream(stream: &mut VSLStreamStatePtr, brng: i32, seed: u32) -> i32 {
    unsafe { sys::vslNewStream(stream as *mut VSLStreamStatePtr, brng, seed) }
}

pub fn vsl_delete_stream(stream: &mut VSLStreamStatePtr) -> i32 {
    unsafe { sys::vslDeleteStream(stream as *mut VSLStreamStatePtr) }
}


pub fn vs_rng_uniform(method: i32, stream: VSLStreamStatePtr, n: i32, r: *mut f32, a: f32, b: f32) -> i32 {
    unsafe { sys::vsRngUniform(method, stream, n, r, a, b) }
}

pub fn vd_rng_uniform(method: i32, stream: VSLStreamStatePtr, n: i32, r: *mut f64, a: f64, b: f64) -> i32 {
    unsafe { sys::vdRngUniform(method, stream, n, r, a, b) }
}


pub struct Buffer<T> {
    data: *mut T,
    len: usize,
}

impl<T> Buffer<T> {
    pub fn new(len: usize, align: i32) -> Buffer<T> {
        Buffer {
            data: malloc(len, align),
            len: len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.data
    }

    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data, self.len) }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        free(self.data);
    }
}

pub struct VSLStream {
    state: *mut c_void
}

impl VSLStream {
    pub fn new(brng: i32, seed: u32) -> VSLStream {
        let mut state = std::ptr::null_mut();
        let status = vsl_new_stream(&mut state, brng, seed); // TODO

        VSLStream {
            state: state,
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.state
    }

    pub fn as_mut_ptr(&self) -> *mut c_void {
        self.state
    }
}

impl Drop for VSLStream {
    fn drop(&mut self) {
        vsl_delete_stream(&mut self.state);
    }
}


impl Buffer<f32> {
    pub fn rng_uniform(&self, method: i32, stream: &VSLStream, a: f32, b: f32) -> i32 {
        vs_rng_uniform(method, stream.as_mut_ptr(), self.len().try_into().unwrap(), self.as_mut_ptr(), a, b)
    }
}

impl Buffer<f64> {
    pub fn rng_uniform(&self, method: i32, stream: &VSLStream, a: f64, b: f64) -> i32 {
        vd_rng_uniform(method, stream.as_mut_ptr(), self.len().try_into().unwrap(), self.as_mut_ptr(), a, b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc() {
        let ptr: *const i32 = malloc(8, 64);
        assert!(!ptr.is_null());
        free(ptr);
    }

    #[test]
    fn test_malloc_2_ptrs() {
        let ptr1: *const i32 = malloc(8, 64);
        let ptr2: *const i32 = malloc(8, 64);

        assert_ne!(ptr1, ptr2);

        free(ptr1);
        free(ptr2);
    }

    #[test]
    fn test_free() {
        let ptr1: *const i32 = malloc(8, 64);

        free(ptr1);
        
        let ptr2: *const i32 = malloc(8, 64);

        assert_eq!(ptr1, ptr2);
        
        free(ptr2);
    }

    #[test]
    fn test_vml_get_mode() {
        let mode = vml_get_mode();
        assert_eq!(mode, VML_HA | VML_FTZDAZ_CURRENT | VML_ERRMODE_DEFAULT);
    }

    #[test]
    fn test_vml_set_mode() {
        let new_mode = VML_LA | VML_FTZDAZ_ON | VML_ERRMODE_IGNORE;
        let old_mode = vml_set_mode(new_mode);

        assert_eq!(old_mode, VML_HA | VML_FTZDAZ_CURRENT | VML_ERRMODE_DEFAULT);

        let mode = vml_get_mode();

        assert_eq!(mode, new_mode);
    }

    #[test]
    fn test_vsl_new_stream() {
        let mut stream: VSLStreamStatePtr = std::ptr::null_mut();
        let status = vsl_new_stream(&mut stream, VSL_BRNG_PHILOX4X32X10, 21);

        assert!(!stream.is_null());
        assert_eq!(status, VSL_STATUS_OK);

        let status = vsl_delete_stream(&mut stream);

        assert!(stream.is_null());
        assert_eq!(status, VSL_STATUS_OK);

        free_buffers();
    }

    #[test]
    fn test_vd_rng_uniform() {
        let len = 8;
        let data: *mut f64 = malloc(len, 64);

        let mut stream: VSLStreamStatePtr = std::ptr::null_mut();
        vsl_new_stream(&mut stream, VSL_BRNG_PHILOX4X32X10, 21);

        let status = vd_rng_uniform(VSL_RNG_METHOD_UNIFORM_STD, stream, len.try_into().unwrap(), data, 0.0, 1.0);

        assert_eq!(status, VSL_STATUS_OK);

        vsl_delete_stream(&mut stream);
        free_buffers();

        assert_eq!(unsafe { *data.offset(len as isize - 1) }, 0.969321598066017);

        free(data);
    }

    #[test]
    fn test_buffer() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let mut stream: VSLStreamStatePtr = std::ptr::null_mut();
        vsl_new_stream(&mut stream, VSL_BRNG_PHILOX4X32X10, 21);

        vd_rng_uniform(VSL_RNG_METHOD_UNIFORM_STD, stream, buf.len().try_into().unwrap(), buf.as_mut_ptr(), 0.0, 1.0);

        vsl_delete_stream(&mut stream);
        free_buffers();

        assert_eq!(buf.as_slice()[buf.len() - 1], 0.969321598066017);
    }

    #[test]
    fn test_vsl_stream() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let stream = VSLStream::new(VSL_BRNG_PHILOX4X32X10, 21);

        vd_rng_uniform(VSL_RNG_METHOD_UNIFORM_STD, stream.as_mut_ptr(), buf.len().try_into().unwrap(), buf.as_mut_ptr(), 0.0, 1.0);

        free_buffers();

        assert_eq!(buf.as_slice()[buf.len() - 1], 0.969321598066017);
    }

    #[test]
    fn test_buffer_rng_uniform() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let stream = VSLStream::new(VSL_BRNG_PHILOX4X32X10, 21);

        buf.rng_uniform(VSL_RNG_METHOD_UNIFORM_STD, &stream, 0.0, 1.0);

        free_buffers();

        assert_eq!(buf.as_slice()[buf.len() - 1], 0.969321598066017);
    }
}
