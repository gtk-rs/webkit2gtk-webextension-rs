// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLParagraphElement(Object<ffi::WebKitDOMHTMLParagraphElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_paragraph_element_get_type(),
    }
}

impl DOMHTMLParagraphElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_paragraph_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_paragraph_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
