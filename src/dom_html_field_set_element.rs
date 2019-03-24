use std::ffi::CString;
use std::ptr::null_mut;

use glib::translate::ToGlibPtr;
use gobject_sys::g_object_get;
use libc::c_void;
use webkit2_webextension_sys;

use super::DOMHTMLFieldSetElement;

pub trait DOMHTMLFieldSetElementExtManual {
    fn get_disabled(&self) -> bool;
}

impl DOMHTMLFieldSetElementExtManual for DOMHTMLFieldSetElement {
    fn get_disabled(&self) -> bool {
        let property_name = CString::new("disabled").unwrap();
        let mut value = 0;
        let element: *mut webkit2_webextension_sys::WebKitDOMHTMLFieldSetElement = self.to_glib_none().0;
        unsafe {
            g_object_get(element as *mut _, property_name.as_ptr(), &mut value as *mut _, null_mut() as *mut c_void);
        }
        value != 0
    }
}
