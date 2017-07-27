// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use DOMCSSStyleSheet;
use DOMObject;
use Error;
use ffi;
use glib::Value;
use glib::translate::*;
use gobject_ffi;
use libc;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSRule(Object<ffi::WebKitDOMCSSRule>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_rule_get_type(),
    }
}

impl DOMCSSRule {
    pub fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_css_text(self.to_glib_none().0))
        }
    }

    pub fn get_parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_rule(self.to_glib_none().0))
        }
    }

    pub fn get_parent_style_sheet(&self) -> Option<DOMCSSStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_style_sheet(self.to_glib_none().0))
        }
    }

    pub fn get_rule_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_css_rule_get_rule_type(self.to_glib_none().0)
        }
    }

    pub fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_rule_set_css_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_property_type(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
