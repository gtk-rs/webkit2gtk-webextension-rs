// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CString;
use std::ptr::null_mut;

use glib::translate::ToGlibPtr;
use gobject_sys::g_object_get;
use libc::c_void;

use super::DOMHTMLFieldSetElement;

pub trait DOMHTMLFieldSetElementExtManual {
    fn get_disabled(&self) -> bool;
}

impl DOMHTMLFieldSetElementExtManual for DOMHTMLFieldSetElement {
    fn get_disabled(&self) -> bool {
        let property_name = CString::new("disabled").unwrap();
        let mut value = 0;
        let element: *mut ffi::WebKitDOMHTMLFieldSetElement = self.to_glib_none().0;
        unsafe {
            g_object_get(
                element as *mut _,
                property_name.as_ptr(),
                &mut value as *mut _,
                null_mut() as *mut c_void,
            );
        }
        value != 0
    }
}
