// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMCharacterData;
use crate::DOMEventTarget;
use crate::DOMNode;
use crate::DOMObject;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMComment")]
    pub struct DOMComment(Object<ffi::WebKitDOMComment, ffi::WebKitDOMCommentClass>) @extends DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_comment_get_type(),
    }
}

impl DOMComment {
        pub const NONE: Option<&'static DOMComment> = None;
    
}

impl fmt::Display for DOMComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMComment")
    }
}
