use libc::c_char;
use std::ffi::CString;

extern {
  pub fn unzip( zip_path: *const c_char
              , target_path: *const c_char ) -> i32;
}

pub fn extract(zip_path: &str, target_path: &str) -> i32 {
  let src = CString::new(zip_path)
    .expect(format!("CString::new(\"{}\") failed", zip_path).as_str());
  let dst = CString::new(target_path)
    .expect(format!("CString::new(\"{}\") failed", target_path).as_str());
  unsafe {
    unzip(src.as_ptr(), dst.as_ptr())
  }
}
