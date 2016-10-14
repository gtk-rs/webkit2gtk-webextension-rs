use glib::IsA;
use glib::translate::{ToGlibPtr, from_glib_full, from_glib_none};

use super::{DOMCSSStyleDeclaration, DOMDOMWindow, DOMElement};
use dom_dom_selection::DOMDOMSelection;

pub trait DOMDOMWindowExtManual {
    fn get_computed_style<T: IsA<DOMElement>>(&self, element: &T, pseudo_element: Option<&str>) -> Option<DOMCSSStyleDeclaration>;
    fn get_selection(&self) -> Option<DOMDOMSelection>;
}

impl DOMDOMWindowExtManual for DOMDOMWindow {
    fn get_computed_style<T: IsA<DOMElement>>(&self, element: &T, pseudo_element: Option<&str>) -> Option<DOMCSSStyleDeclaration> {
        unsafe {
            from_glib_full(sys::webkit_dom_dom_window_get_computed_style(self.to_glib_none().0, element.to_glib_none().0, pseudo_element.to_glib_none().0))
        }
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
