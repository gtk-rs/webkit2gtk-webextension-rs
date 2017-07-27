// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::ptr;

glib_wrapper! {
    pub struct DOMText(Object<ffi::WebKitDOMText>): DOMCharacterData, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_text_get_type(),
    }
}

pub trait DOMTextExt {
    fn get_whole_text(&self) -> Option<String>;

    fn replace_whole_text(&self, content: &str) -> Result<DOMText, Error>;

    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, Error>;
}

impl<O: IsA<DOMText>> DOMTextExt for O {
    fn get_whole_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_text_get_whole_text(self.to_glib_none().0))
        }
    }

    fn replace_whole_text(&self, content: &str) -> Result<DOMText, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_text_replace_whole_text(self.to_glib_none().0, content.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_text_split_text(self.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
