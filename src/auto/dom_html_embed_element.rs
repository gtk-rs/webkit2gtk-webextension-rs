// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLEmbedElement(Object<ffi::WebKitDOMHTMLEmbedElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_embed_element_get_type(),
    }
}

impl DOMHTMLEmbedElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_embed_element_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_src(self.to_glib_none().0))
        }
    }

    pub fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_type_attr(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_embed_element_get_width(self.to_glib_none().0)
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_height(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_height(self.to_glib_none().0, value);
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_width(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_width(self.to_glib_none().0, value);
        }
    }
}