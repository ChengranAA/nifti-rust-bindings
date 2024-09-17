use crate::nifti::nifti::NiftiImage; 
extern crate libc;
use std::ffi::CString;
 
extern "C" {
    fn nifti_image_read(hname: *const libc::c_char, read_data: i32) -> *mut NiftiImage;
}

// Rust API for Nifti Read
pub fn read_nifti_image(hname: &str, read_data: i32) -> Option<Box<NiftiImage>> {
    let c_hname = CString::new(hname).expect("CString::new failed");
    unsafe {
        let image_ptr = nifti_image_read(c_hname.as_ptr(), read_data);
        if image_ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(image_ptr))
        }
    }
}

