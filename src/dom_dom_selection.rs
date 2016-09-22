use DOMNode;
use DOMObject;
use glib::translate::{ToGlibPtr, from_glib_none};

glib_wrapper! {
    pub struct DOMDOMSelection(Object<sys::WebKitDOMDOMSelection>): DOMObject;

    match fn {
        get_type => || sys::webkit_dom_dom_selection_get_type(),
    }
}

impl DOMDOMSelection {
    pub fn empty(&self) {
        unsafe {
            sys::webkit_dom_dom_selection_empty(self.to_glib_none().0);
        }
    }

    pub fn get_anchor_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(sys::webkit_dom_dom_selection_get_anchor_node(self.to_glib_none().0))
        }
    }
}

pub mod sys {
    use ffi;
    use glib_ffi::GType;
    use libc::c_void;

    #[repr(C)]
    pub struct WebKitDOMDOMSelection(c_void);

    extern "C" {
        pub fn webkit_dom_dom_selection_empty(selection: *mut WebKitDOMDOMSelection);
        pub fn webkit_dom_dom_selection_get_anchor_node(selection: *mut WebKitDOMDOMSelection) -> *mut ffi::WebKitDOMNode;
        pub fn webkit_dom_dom_selection_get_type() -> GType;
    }
}
