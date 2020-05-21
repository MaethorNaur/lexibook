use std::convert::From;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct StringList {
    pub items: *const *const c_char,
    pub length: u64,
}
#[no_mangle]
pub extern "C" fn lexibook_string_list_free(ptr: *mut StringList) {
    let string_list = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    string_list.free();
    unsafe {
        Box::from_raw(ptr);
    }
}

impl StringList {
    pub fn free(&self) {
        unsafe {
            let v: Vec<*const c_char> = Vec::from_raw_parts(
                self.items as *mut *const c_char,
                self.length as usize,
                self.length as usize,
            );
            for ptr in &v {
                if ptr.is_null() {
                    continue;
                }
                let _ = CString::from_raw(*ptr as *mut c_char);
            }
        }
    }
}

impl From<Vec<String>> for StringList {
    fn from(vec: Vec<String>) -> Self {
        let mut strings: Vec<*const c_char> = vec
            .into_iter()
            .map(|s| {
                let cstr = CString::new(s).unwrap();
                cstr.into_raw() as *const c_char
            })
            .collect();
        strings.shrink_to_fit();
        let array = StringList {
            items: strings.as_ptr() as *const *const c_char,
            length: strings.len() as u64,
        };
        std::mem::forget(strings);
        array
    }
}
