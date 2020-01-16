use std::convert::From;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct StringList {
    pub items: *const *const c_char,
    pub length: usize,
}
#[no_mangle]
pub extern "C" fn lexibook_string_list_free(string_list: StringList) {
    string_list.free()
}

impl StringList {
    pub fn free(self) {
        unsafe {
            let v: Vec<*const c_char> =
                Vec::from_raw_parts(self.items as *mut *const c_char, self.length, self.length);
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
            length: strings.len(),
        };
        std::mem::forget(strings);
        array
    }
}
