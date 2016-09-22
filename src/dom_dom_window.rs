use glib::translate::{ToGlibPtr, from_glib_none};

use super::DOMDOMWindow;
use dom_dom_selection::DOMDOMSelection;

pub trait DOMDOMWindowExtManual {
    fn get_selection(&self) -> Option<DOMDOMSelection>;
}

impl DOMDOMWindowExtManual for DOMDOMWindow {
    fn get_selection(&self) -> Option<DOMDOMSelection> {
        unsafe { from_glib_none(sys::webkit_dom_dom_window_get_selection(self.to_glib_none().0)) }
    }
}

mod sys {
    use ffi;

    use dom_dom_selection::sys::WebKitDOMDOMSelection;

    extern "C" {
        pub fn webkit_dom_dom_window_get_selection(window: *mut ffi::WebKitDOMDOMWindow) -> *mut WebKitDOMDOMSelection;
    }
}
