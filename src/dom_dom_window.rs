use std::ffi::CString;
use std::ptr::null_mut;

use ffi;
use glib::IsA;
use glib::translate::{FromGlib, ToGlibPtr, from_glib_full, from_glib_none};
use gobject_sys::g_object_get;
use libc::c_void;

use super::{DOMCSSStyleDeclaration, DOMDOMWindow, DOMElement};
use dom_dom_selection::DOMDOMSelection;

pub trait DOMDOMWindowExtManual {
    fn get_computed_style<T: IsA<DOMElement>>(&self, element: &T, pseudo_element: Option<&str>) -> Option<DOMCSSStyleDeclaration>;
    fn get_frame_element(&self) -> Option<DOMElement>;
    fn get_inner_height(&self) -> i64;
    fn get_inner_width(&self) -> i64;
    fn get_selection(&self) -> Option<DOMDOMSelection>;
}

impl DOMDOMWindow {
    fn get_long_property(&self, property_name: &str) -> i64 {
        let property_name = CString::new(property_name).unwrap();
        let mut value = 0;;
        let window: *mut ffi::WebKitDOMDOMWindow = self.to_glib_none().0;
        unsafe {
            g_object_get(window as *mut _, property_name.as_ptr(), &mut value as *mut _, null_mut() as *mut c_void);
        }
        value
    }
}

impl DOMDOMWindowExtManual for DOMDOMWindow {
    fn get_computed_style<T: IsA<DOMElement>>(&self, element: &T, pseudo_element: Option<&str>) -> Option<DOMCSSStyleDeclaration> {
        unsafe {
            from_glib_full(sys::webkit_dom_dom_window_get_computed_style(self.to_glib_none().0, element.to_glib_none().0, pseudo_element.to_glib_none().0))
        }
    }

    fn get_frame_element(&self) -> Option<DOMElement> {
        let property_name = CString::new("frame-element").unwrap();
        let mut value: *mut ffi::WebKitDOMElement = null_mut();;
        let window: *mut ffi::WebKitDOMDOMWindow = self.to_glib_none().0;
        unsafe {
            g_object_get(window as *mut _, property_name.as_ptr(), &mut value as *mut _, null_mut() as *mut c_void);
        }
        unsafe { from_glib_none(value) }
    }

    fn get_inner_height(&self) -> i64 {
        self.get_long_property("inner-height")
    }

    fn get_inner_width(&self) -> i64 {
        self.get_long_property("inner-width")
    }

    fn get_selection(&self) -> Option<DOMDOMSelection> {
        unsafe { from_glib_none(sys::webkit_dom_dom_window_get_selection(self.to_glib_none().0)) }
    }
}

mod sys {
    use ffi;

    use libc::c_char;
    use dom_dom_selection::sys::WebKitDOMDOMSelection;

    extern "C" {
        pub fn webkit_dom_dom_window_get_computed_style(window: *mut ffi::WebKitDOMDOMWindow, element: *mut ffi::WebKitDOMElement, pseudo_element: *const c_char) -> *mut ffi::WebKitDOMCSSStyleDeclaration;
        pub fn webkit_dom_dom_window_get_selection(window: *mut ffi::WebKitDOMDOMWindow) -> *mut WebKitDOMDOMSelection;
    }
}
