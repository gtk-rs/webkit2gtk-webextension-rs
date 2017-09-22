// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMDocument;
use DOMElement;
use DOMEventTarget;
use DOMNodeList;
use DOMObject;
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
    pub struct DOMNode(Object<ffi::WebKitDOMNode>): DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_node_get_type(),
    }
}

pub trait DOMNodeExt {
    fn append_child<P: IsA<DOMNode>>(&self, newChild: &P) -> Result<DOMNode, Error>;

    fn clone_node(&self, deep: bool) -> Result<DOMNode, Error>;

    #[cfg(feature = "v2_14")]
    fn clone_node_with_error(&self, deep: bool) -> Result<DOMNode, Error>;

    fn compare_document_position<P: IsA<DOMNode>>(&self, other: &P) -> libc::c_ushort;

    fn contains<P: IsA<DOMNode>>(&self, other: &P) -> bool;

    fn get_base_uri(&self) -> Option<String>;

    fn get_child_nodes(&self) -> Option<DOMNodeList>;

    fn get_first_child(&self) -> Option<DOMNode>;

    fn get_last_child(&self) -> Option<DOMNode>;

    fn get_local_name(&self) -> Option<String>;

    fn get_namespace_uri(&self) -> Option<String>;

    fn get_next_sibling(&self) -> Option<DOMNode>;

    fn get_node_name(&self) -> Option<String>;

    fn get_node_type(&self) -> libc::c_ushort;

    fn get_node_value(&self) -> Option<String>;

    fn get_owner_document(&self) -> Option<DOMDocument>;

    fn get_parent_element(&self) -> Option<DOMElement>;

    fn get_parent_node(&self) -> Option<DOMNode>;

    fn get_prefix(&self) -> Option<String>;

    fn get_previous_sibling(&self) -> Option<DOMNode>;

    fn get_text_content(&self) -> Option<String>;

    fn has_child_nodes(&self) -> bool;

    fn insert_before<'a, P: IsA<DOMNode>, Q: IsA<DOMNode> + 'a, R: Into<Option<&'a Q>>>(&self, newChild: &P, refChild: R) -> Result<DOMNode, Error>;

    fn is_default_namespace(&self, namespaceURI: &str) -> bool;

    fn is_equal_node<P: IsA<DOMNode>>(&self, other: &P) -> bool;

    fn is_same_node<P: IsA<DOMNode>>(&self, other: &P) -> bool;

    fn is_supported(&self, feature: &str, version: &str) -> bool;

    fn lookup_namespace_uri(&self, prefix: &str) -> Option<String>;

    fn lookup_prefix(&self, namespaceURI: &str) -> Option<String>;

    fn normalize(&self);

    fn remove_child<P: IsA<DOMNode>>(&self, oldChild: &P) -> Result<DOMNode, Error>;

    fn replace_child<P: IsA<DOMNode>, Q: IsA<DOMNode>>(&self, newChild: &P, oldChild: &Q) -> Result<DOMNode, Error>;

    fn set_node_value(&self, value: &str) -> Result<(), Error>;

    fn set_prefix(&self, value: &str) -> Result<(), Error>;

    fn set_text_content(&self, value: &str) -> Result<(), Error>;

    fn connect_property_base_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_child_nodes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_next_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_node_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_node_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_owner_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_parent_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_parent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_previous_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_text_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMNode> + IsA<glib::object::Object>> DOMNodeExt for O {
    fn append_child<P: IsA<DOMNode>>(&self, newChild: &P) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_append_child(self.to_glib_none().0, newChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn clone_node(&self, deep: bool) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_clone_node(self.to_glib_none().0, deep.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_14")]
    fn clone_node_with_error(&self, deep: bool) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_clone_node_with_error(self.to_glib_none().0, deep.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_document_position<P: IsA<DOMNode>>(&self, other: &P) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_node_compare_document_position(self.to_glib_none().0, other.to_glib_none().0)
        }
    }

    fn contains<P: IsA<DOMNode>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_contains(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn get_base_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_base_uri(self.to_glib_none().0))
        }
    }

    fn get_child_nodes(&self) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_child_nodes(self.to_glib_none().0))
        }
    }

    fn get_first_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_first_child(self.to_glib_none().0))
        }
    }

    fn get_last_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_last_child(self.to_glib_none().0))
        }
    }

    fn get_local_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_local_name(self.to_glib_none().0))
        }
    }

    fn get_namespace_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_namespace_uri(self.to_glib_none().0))
        }
    }

    fn get_next_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_next_sibling(self.to_glib_none().0))
        }
    }

    fn get_node_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_name(self.to_glib_none().0))
        }
    }

    fn get_node_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_node_get_node_type(self.to_glib_none().0)
        }
    }

    fn get_node_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_value(self.to_glib_none().0))
        }
    }

    fn get_owner_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_owner_document(self.to_glib_none().0))
        }
    }

    fn get_parent_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_element(self.to_glib_none().0))
        }
    }

    fn get_parent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_node(self.to_glib_none().0))
        }
    }

    fn get_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_prefix(self.to_glib_none().0))
        }
    }

    fn get_previous_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_previous_sibling(self.to_glib_none().0))
        }
    }

    fn get_text_content(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_text_content(self.to_glib_none().0))
        }
    }

    fn has_child_nodes(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_has_child_nodes(self.to_glib_none().0))
        }
    }

    fn insert_before<'a, P: IsA<DOMNode>, Q: IsA<DOMNode> + 'a, R: Into<Option<&'a Q>>>(&self, newChild: &P, refChild: R) -> Result<DOMNode, Error> {
        let refChild = refChild.into();
        let refChild = refChild.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_insert_before(self.to_glib_none().0, newChild.to_glib_none().0, refChild.0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_default_namespace(&self, namespaceURI: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_default_namespace(self.to_glib_none().0, namespaceURI.to_glib_none().0))
        }
    }

    fn is_equal_node<P: IsA<DOMNode>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_equal_node(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn is_same_node<P: IsA<DOMNode>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_same_node(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_supported(self.to_glib_none().0, feature.to_glib_none().0, version.to_glib_none().0))
        }
    }

    fn lookup_namespace_uri(&self, prefix: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_namespace_uri(self.to_glib_none().0, prefix.to_glib_none().0))
        }
    }

    fn lookup_prefix(&self, namespaceURI: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_prefix(self.to_glib_none().0, namespaceURI.to_glib_none().0))
        }
    }

    fn normalize(&self) {
        unsafe {
            ffi::webkit_dom_node_normalize(self.to_glib_none().0);
        }
    }

    fn remove_child<P: IsA<DOMNode>>(&self, oldChild: &P) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_remove_child(self.to_glib_none().0, oldChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn replace_child<P: IsA<DOMNode>, Q: IsA<DOMNode>>(&self, newChild: &P, oldChild: &Q) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_replace_child(self.to_glib_none().0, newChild.to_glib_none().0, oldChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_node_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_node_value(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_prefix(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_prefix(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_text_content(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_text_content(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_base_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::base-uri",
                transmute(notify_base_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_child_nodes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::child-nodes",
                transmute(notify_child_nodes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::first-child",
                transmute(notify_first_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::last-child",
                transmute(notify_last_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_next_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::next-sibling",
                transmute(notify_next_sibling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_node_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::node-name",
                transmute(notify_node_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_node_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::node-type",
                transmute(notify_node_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::node-value",
                transmute(notify_node_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_owner_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::owner-document",
                transmute(notify_owner_document_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent-element",
                transmute(notify_parent_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent-node",
                transmute(notify_parent_node_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_previous_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::previous-sibling",
                transmute(notify_previous_sibling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-content",
                transmute(notify_text_content_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_base_uri_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_child_nodes_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_first_child_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_last_child_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_next_sibling_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_node_name_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_node_type_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_node_value_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_owner_document_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_element_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_node_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_previous_sibling_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_content_trampoline<P>(this: *mut ffi::WebKitDOMNode, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNode> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNode::from_glib_borrow(this).downcast_unchecked())
}
