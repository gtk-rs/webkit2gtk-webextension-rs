// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMCSSRule;
use DOMCSSRuleList;
use DOMObject;
use DOMStyleSheet;
use Error;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSStyleSheet(Object<ffi::WebKitDOMCSSStyleSheet>): DOMStyleSheet, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_style_sheet_get_type(),
    }
}

pub trait DOMCSSStyleSheetExt {
    fn add_rule(&self, selector: &str, style: &str, index: libc::c_ulong) -> Result<libc::c_long, Error>;

    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), Error>;

    fn get_css_rules(&self) -> Option<DOMCSSRuleList>;

    fn get_owner_rule(&self) -> Option<DOMCSSRule>;

    fn get_rules(&self) -> Option<DOMCSSRuleList>;

    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, Error>;

    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), Error>;

    fn connect_property_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMCSSStyleSheet> + IsA<glib::object::Object>> DOMCSSStyleSheetExt for O {
    fn add_rule(&self, selector: &str, style: &str, index: libc::c_ulong) -> Result<libc::c_long, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_add_rule(self.to_glib_none().0, selector.to_glib_none().0, style.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_delete_rule(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_css_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_css_rules(self.to_glib_none().0))
        }
    }

    fn get_owner_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_owner_rule(self.to_glib_none().0))
        }
    }

    fn get_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_rules(self.to_glib_none().0))
        }
    }

    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_insert_rule(self.to_glib_none().0, rule.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_remove_rule(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::css-rules",
                transmute(notify_css_rules_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::owner-rule",
                transmute(notify_owner_rule_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rules",
                transmute(notify_rules_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_css_rules_trampoline<P>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSStyleSheet> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_owner_rule_trampoline<P>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSStyleSheet> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rules_trampoline<P>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSStyleSheet> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCSSStyleSheet::from_glib_borrow(this).downcast_unchecked())
}
