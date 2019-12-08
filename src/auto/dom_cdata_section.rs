// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use DOMText;
use glib::translate::*;
use std::fmt;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMCDATASection(Object<webkit2_webextension_sys::WebKitDOMCDATASection, webkit2_webextension_sys::WebKitDOMCDATASectionClass, DOMCDATASectionClass>) @extends DOMText, DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_cdata_section_get_type(),
    }
}

impl DOMCDATASection {}

pub const NONE_DOMCDATA_SECTION: Option<&DOMCDATASection> = None;

impl fmt::Display for DOMCDATASection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMCDATASection")
    }
}
