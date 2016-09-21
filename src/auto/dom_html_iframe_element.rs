// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMDOMWindow;
use DOMDocument;
use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLIFrameElement(Object<ffi::WebKitDOMHTMLIFrameElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_iframe_element_get_type(),
    }
}

impl DOMHTMLIFrameElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_iframe_element_get_content_document(self.to_glib_none().0))
        }
    }

    pub fn get_content_window(&self) -> Option<DOMDOMWindow> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_content_window(self.to_glib_none().0))
        }
    }

    pub fn get_frame_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_frame_border(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_height(self.to_glib_none().0))
        }
    }

    pub fn get_long_desc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_long_desc(self.to_glib_none().0))
        }
    }

    pub fn get_margin_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_margin_height(self.to_glib_none().0))
        }
    }

    pub fn get_margin_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_margin_width(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_scrolling(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_scrolling(self.to_glib_none().0))
        }
    }

    pub fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_src(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_iframe_element_get_width(self.to_glib_none().0))
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_frame_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_frame_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_long_desc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_margin_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_margin_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_margin_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_margin_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_scrolling(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_scrolling(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_iframe_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}