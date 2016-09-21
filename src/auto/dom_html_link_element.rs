// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use DOMStyleSheet;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLLinkElement(Object<ffi::WebKitDOMHTMLLinkElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_link_element_get_type(),
    }
}

impl DOMHTMLLinkElement {
    pub fn get_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_charset(self.to_glib_none().0))
        }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_link_element_get_disabled(self.to_glib_none().0))
        }
    }

    pub fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_href(self.to_glib_none().0))
        }
    }

    pub fn get_hreflang(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_hreflang(self.to_glib_none().0))
        }
    }

    pub fn get_media(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_media(self.to_glib_none().0))
        }
    }

    pub fn get_rel(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_rel(self.to_glib_none().0))
        }
    }

    pub fn get_rev(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_rev(self.to_glib_none().0))
        }
    }

    pub fn get_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_sheet(self.to_glib_none().0))
        }
    }

    pub fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_target(self.to_glib_none().0))
        }
    }

    pub fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_type_attr(self.to_glib_none().0))
        }
    }

    pub fn set_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_href(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_hreflang(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_hreflang(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_media(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_media(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_rel(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_rel(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_rev(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_rev(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}