use std::{ptr, slice, ffi::c_void, mem::size_of, ops::{Deref, DerefMut}};
use num_enum::{IntoPrimitive, TryFromPrimitive};


#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod sys;


#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum VmlAccuracyMode {
    HighAccuracy = sys::VML_HA,
    LowAccuracy = sys::VML_LA,
    EnhancedPerformance = sys::VML_EP,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum VmlFtzdazMode {
    On = sys::VML_FTZDAZ_ON,
    Off = sys::VML_FTZDAZ_OFF,
    Current = sys::VML_FTZDAZ_CURRENT,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum VmlErrorMode {
    Ignore = sys::VML_ERRMODE_IGNORE,
    NoError = sys::VML_ERRMODE_NOERR,
    Errno = sys::VML_ERRMODE_ERRNO,
    Stderr = sys::VML_ERRMODE_STDERR,
    Exception = sys::VML_ERRMODE_EXCEPT,
    Callback = sys::VML_ERRMODE_CALLBACK,
    Default = sys::VML_ERRMODE_DEFAULT,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VmlMode {
    accuracy: VmlAccuracyMode,
    ftzdaz: VmlFtzdazMode,
    error: VmlErrorMode,
}

impl Into<u32> for VmlMode {
    fn into(self) -> u32 {
        let accuracy: u32 = self.accuracy.into();
        let ftzdaz: u32 = self.ftzdaz.into();
        let error: u32 = self.error.into();
        
        accuracy | ftzdaz | error
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VmlModeTryFromError {
    InvalidAccuracyMode(u32),
    InvalidFtzdazMode(u32),
    InvalidErrorMode(u32),
}

impl TryFrom<u32> for VmlMode {
    type Error = VmlModeTryFromError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let accuracy = match VmlAccuracyMode::try_from(value & sys::VML_ACCURACY_MASK) {
            Ok(mode) => mode,
            Err(error) => return Err(Self::Error::InvalidAccuracyMode(error.number)),
        };
        let ftzdaz = match VmlFtzdazMode::try_from(value & sys::VML_FTZDAZ_MASK) {
            Ok(mode) => mode,
            Err(error) => return Err(Self::Error::InvalidFtzdazMode(error.number)),
        };
        let error = match VmlErrorMode::try_from(value & sys::VML_ERRMODE_MASK) {
            Ok(mode) => mode,
            Err(error) => return Err(Self::Error::InvalidErrorMode(error.number)),
        };

        Ok(Self {
            accuracy,
            ftzdaz,
            error,
        })
    }
}


pub type VslStreamState = c_void;


#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum VslBrng {
    MCG31 = sys::VSL_BRNG_MCG31,
    R250 = sys::VSL_BRNG_R250,
    MRG32k3a = sys::VSL_BRNG_MRG32K3A,
    MCG59 = sys::VSL_BRNG_MCG59,
    WichmannHill = sys::VSL_BRNG_WH,
    MT19937 = sys::VSL_BRNG_MT19937,
    MT2203 = sys::VSL_BRNG_MT2203,
    SFMT19937 = sys::VSL_BRNG_SFMT19937,
    Sobol = sys::VSL_BRNG_SOBOL,
    Niederreiter = sys::VSL_BRNG_NIEDERR,
    IntegerAbstract = sys::VSL_BRNG_IABSTRACT,
    DoubleAbstract = sys::VSL_BRNG_DABSTRACT,
    SingleAbstract = sys::VSL_BRNG_SABSTRACT,
    NonDeterministic = sys::VSL_BRNG_NONDETERM,
    Philox4x32x10 = sys::VSL_BRNG_PHILOX4X32X10,
    ARS5 = sys::VSL_BRNG_ARS5,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum VslError {
    FeatureNotImplemented = sys::VSL_ERROR_FEATURE_NOT_IMPLEMENTED,
    Unknown = sys::VSL_ERROR_UNKNOWN,
    BadArguments = sys::VSL_ERROR_BADARGS,
    MemoryFailure = sys::VSL_ERROR_MEM_FAILURE,
    NullPointer = sys::VSL_ERROR_NULL_PTR,
    CpuNotSupported = sys::VSL_ERROR_CPU_NOT_SUPPORTED,
    InvalidBrngIndex = sys::VSL_RNG_ERROR_INVALID_BRNG_INDEX,
    LeapfrogUnsupported = sys::VSL_RNG_ERROR_LEAPFROG_UNSUPPORTED,
    SkipaheadUnsupported = sys::VSL_RNG_ERROR_SKIPAHEAD_UNSUPPORTED,
    SkipaheadXUnsupported = sys::VSL_RNG_ERROR_SKIPAHEADEX_UNSUPPORTED,
    BrngsIncompatible = sys::VSL_RNG_ERROR_BRNGS_INCOMPATIBLE,
    BadStream = sys::VSL_RNG_ERROR_BAD_STREAM,
    BrngTableFull = sys::VSL_RNG_ERROR_BRNG_TABLE_FULL,
    BatStreamStateSize = sys::VSL_RNG_ERROR_BAD_STREAM_STATE_SIZE,
    BadWordSize = sys::VSL_RNG_ERROR_BAD_WORD_SIZE,
    BadNSeeds = sys::VSL_RNG_ERROR_BAD_NSEEDS,
    BadNBits = sys::VSL_RNG_ERROR_BAD_NBITS,
    QrngPeriodElapsed = sys::VSL_RNG_ERROR_QRNG_PERIOD_ELAPSED,
    LeapfrogNStreamsTooBig = sys::VSL_RNG_ERROR_LEAPFROG_NSTREAMS_TOO_BIG,
    BrngNotSupported = sys::VSL_RNG_ERROR_BRNG_NOT_SUPPORTED,
    BadUpdate = sys::VSL_RNG_ERROR_BAD_UPDATE,
    NoNumbers = sys::VSL_RNG_ERROR_NO_NUMBERS,
    InvalidAbstractStream = sys::VSL_RNG_ERROR_INVALID_ABSTRACT_STREAM,
    NonDeteministicNotSupported = sys::VSL_RNG_ERROR_NONDETERM_NOT_SUPPORTED,
    NonDeteministicNRetriesExceeded = sys::VSL_RNG_ERROR_NONDETERM_NRETRIES_EXCEEDED,
    ARS5NotSupported = sys::VSL_RNG_ERROR_ARS5_NOT_SUPPORTED,
    FileClose = sys::VSL_RNG_ERROR_FILE_CLOSE,
    FileOpen = sys::VSL_RNG_ERROR_FILE_OPEN,
    FileWrite = sys::VSL_RNG_ERROR_FILE_WRITE,
    FileRead = sys::VSL_RNG_ERROR_FILE_READ,
    BadFileFormat = sys::VSL_RNG_ERROR_BAD_FILE_FORMAT,
    UnsupportedFileVersion = sys::VSL_RNG_ERROR_UNSUPPORTED_FILE_VER,
    BadMemoryFormat = sys::VSL_RNG_ERROR_BAD_MEM_FORMAT,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum VslRngMethodUniform {
    Fast = sys::VSL_RNG_METHOD_UNIFORM_STD,
    Accurate = sys::VSL_RNG_METHOD_UNIFORM_STD_ACCURATE,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum VslRngMethodUniformDiscrete {
    Fast = sys::VSL_RNG_METHOD_UNIFORM_STD,
}


pub fn malloc<T>(n: usize, align: i32) -> *mut T {
    unsafe { sys::MKL_malloc(n * size_of::<T>(), align) as *mut T }
}

pub fn free<T>(ptr: *const T) {
    unsafe { sys::MKL_free(ptr as *mut c_void) };
}

pub fn free_buffers() {
    unsafe { sys::MKL_Free_Buffers() };
}


pub fn vml_get_mode() -> VmlMode {
    VmlMode::try_from(unsafe { sys::vmlGetMode() }).unwrap()
}

pub fn vml_set_mode(new_mode: VmlMode) -> VmlMode {
    VmlMode::try_from(unsafe { sys::vmlSetMode(new_mode.into()) }).unwrap()
}


pub fn vsl_new_stream(brng: VslBrng, seed: u32) -> Result<*mut VslStreamState, VslError> {
    let mut stream: *mut VslStreamState = ptr::null_mut();
    match unsafe { sys::vslNewStream(&mut stream as *mut *mut VslStreamState, brng.into(), seed) } {
        sys::VSL_STATUS_OK => Ok(stream),
        error => Err(VslError::try_from(error).unwrap()),
    }
}

pub fn vsl_delete_stream(stream: &mut *mut VslStreamState) -> Result<(), VslError> {
    match unsafe { sys::vslDeleteStream(stream as *mut *mut VslStreamState) } {
        sys::VSL_STATUS_OK => Ok(()),
        error => Err(VslError::try_from(error).unwrap()),
    }
}


pub fn vs_rng_uniform(method: VslRngMethodUniform, stream: *mut VslStreamState, n: i32, r: *mut f32, a: f32, b: f32) -> Result<(), VslError> {
    match unsafe { sys::vsRngUniform(method.into(), stream, n, r, a, b) } {
        sys::VSL_STATUS_OK => Ok(()),
        error => Err(VslError::try_from(error).unwrap()),
    }
}

pub fn vd_rng_uniform(method: VslRngMethodUniform, stream: *mut VslStreamState, n: i32, r: *mut f64, a: f64, b: f64) -> Result<(), VslError> {
    match unsafe { sys::vdRngUniform(method.into(), stream, n, r, a, b) } {
        sys::VSL_STATUS_OK => Ok(()),
        error => Err(VslError::try_from(error).unwrap()),
    }
}

pub fn vi_rng_uniform(method: VslRngMethodUniformDiscrete, stream: *mut VslStreamState, n: i32, r: *mut i32, a: i32, b: i32) -> Result<(), VslError> {
    match unsafe { sys::viRngUniform(method.into(), stream, n, r, a, b) } {
        sys::VSL_STATUS_OK => Ok(()),
        error => Err(VslError::try_from(error).unwrap()),
    }
}


pub struct Buffer<T> {
    data: *mut T,
    len: usize,
}

impl<T> Buffer<T> {
    pub fn new(len: usize, align: i32) -> Self {
        Self {
            data: malloc(len, align),
            len,
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

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}


pub struct VslStream {
    state: *mut VslStreamState
}

impl VslStream {
    pub fn new(brng: VslBrng, seed: u32) -> Result<Self, VslError> {
        Ok(Self { state: vsl_new_stream(brng, seed)? })
    }

    pub fn as_ptr(&self) -> *const VslStreamState {
        self.state
    }

    pub fn as_mut_ptr(&self) -> *mut VslStreamState {
        self.state
    }
}

impl Drop for VslStream {
    fn drop(&mut self) {
        vsl_delete_stream(&mut self.state).unwrap();
    }
}


impl Buffer<f32> {
    pub fn rng_uniform(&self, method: VslRngMethodUniform, stream: &VslStream, a: f32, b: f32) -> Result<(), VslError> {
        vs_rng_uniform(method, stream.as_mut_ptr(), self.len().try_into().unwrap(), self.as_mut_ptr(), a, b)
    }
}

impl Buffer<f64> {
    pub fn rng_uniform(&self, method: VslRngMethodUniform, stream: &VslStream, a: f64, b: f64) -> Result<(), VslError> {
        vd_rng_uniform(method, stream.as_mut_ptr(), self.len().try_into().unwrap(), self.as_mut_ptr(), a, b)
    }
}

impl Buffer<i32> {
    pub fn rng_uniform(&self, method: VslRngMethodUniformDiscrete, stream: &VslStream, a: i32, b: i32) -> Result<(), VslError> {
        vi_rng_uniform(method, stream.as_mut_ptr(), self.len().try_into().unwrap(), self.as_mut_ptr(), a, b)
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
        assert_eq!(mode, VmlMode {
            accuracy:VmlAccuracyMode::HighAccuracy,
            ftzdaz: VmlFtzdazMode::Current,
            error: VmlErrorMode::Default,
        });
    }

    #[test]
    fn test_vml_set_mode() {
        let new_mode = VmlMode {
            accuracy:VmlAccuracyMode::LowAccuracy,
            ftzdaz: VmlFtzdazMode::On,
            error: VmlErrorMode::Ignore,
        };
        let old_mode = vml_set_mode(new_mode);

        assert_eq!(old_mode, VmlMode {
            accuracy:VmlAccuracyMode::HighAccuracy,
            ftzdaz: VmlFtzdazMode::Current,
            error: VmlErrorMode::Default,
        });

        assert_eq!(vml_get_mode(), new_mode);
    }

    #[test]
    fn test_vsl_new_stream() {
        let mut stream = vsl_new_stream(VslBrng::Philox4x32x10, 21).unwrap();

        assert!(!stream.is_null());

        vsl_delete_stream(&mut stream).unwrap();

        assert!(stream.is_null());

        free_buffers();
    }

    #[test]
    fn test_vd_rng_uniform() {
        let len = 8;
        let data: *mut f64 = malloc(len, 64);

        let mut stream = vsl_new_stream(VslBrng::Philox4x32x10, 21).unwrap();

        vd_rng_uniform(VslRngMethodUniform::Fast, stream, len.try_into().unwrap(), data, 0.0, 1.0).unwrap();

        vsl_delete_stream(&mut stream).unwrap();
        free_buffers();

        assert_eq!(unsafe { *data.offset(len as isize - 1) }, 0.969321598066017);

        free(data);
    }

    #[test]
    fn test_buffer() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let mut stream = vsl_new_stream(VslBrng::Philox4x32x10, 21).unwrap();

        vd_rng_uniform(VslRngMethodUniform::Fast, stream, buf.len().try_into().unwrap(), buf.as_mut_ptr(), 0.0, 1.0).unwrap();

        vsl_delete_stream(&mut stream).unwrap();
        free_buffers();

        assert_eq!(buf.last().unwrap().clone(), 0.969321598066017);
    }

    #[test]
    fn test_vsl_stream() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let stream = VslStream::new(VslBrng::Philox4x32x10, 21).unwrap();

        vd_rng_uniform(VslRngMethodUniform::Fast, stream.as_mut_ptr(), buf.len().try_into().unwrap(), buf.as_mut_ptr(), 0.0, 1.0).unwrap();

        free_buffers();

        assert_eq!(buf.last().unwrap().clone(), 0.969321598066017);
    }

    #[test]
    fn test_buffer_rng_uniform() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let stream = VslStream::new(VslBrng::Philox4x32x10, 21).unwrap();

        buf.rng_uniform(VslRngMethodUniform::Fast, &stream, 0.0, 1.0).unwrap();

        free_buffers();

        assert_eq!(buf.last().unwrap().clone(), 0.969321598066017);
    }

    #[test]
    fn test_multiple_rng() {
        let buf: Buffer<f64> = Buffer::new(8, 64);

        let stream = VslStream::new(VslBrng::Philox4x32x10, 21).unwrap();

        buf.rng_uniform(VslRngMethodUniform::Fast, &stream, 0.0, 1.0).unwrap();
        buf.rng_uniform(VslRngMethodUniform::Fast, &stream, 0.0, 1.0).unwrap();

        free_buffers();

        assert_ne!(buf.last().unwrap().clone(), 0.969321598066017);
    }

    #[test]
    fn test_vsl_error() {
        let mut stream = vsl_new_stream(VslBrng::Philox4x32x10, 21).unwrap();

        vd_rng_uniform(VslRngMethodUniform::Fast, stream, 8, ptr::null_mut(), 0.0, 1.0).unwrap_err();

        vsl_delete_stream(&mut stream).unwrap();
        free_buffers();
    }
}
