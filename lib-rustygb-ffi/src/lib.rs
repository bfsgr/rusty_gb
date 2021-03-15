use libc::c_void;
use rustygb::Gameboy;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn create() -> *mut libc::c_void {
    let game = Gameboy::default();
    let data = Box::into_raw(Box::new(game));

    return data as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn insert(sys: *mut libc::c_void, filename: *const libc::c_char) {
    assert!(!sys.is_null());
    let mut sys = Box::from_raw(sys as *mut Gameboy);

    sys.insert(CStr::from_ptr(filename).to_str().unwrap().to_string())
}
