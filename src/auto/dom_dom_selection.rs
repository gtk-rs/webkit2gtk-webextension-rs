// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use DOMNode;
use DOMObject;
#[cfg(feature = "v2_16")]
use DOMRange;
#[cfg(feature = "v2_16")]
use Error;
use ffi;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
#[cfg(feature = "v2_16")]
use libc;
#[cfg(feature = "v2_16")]
use std::ptr;

glib_wrapper! {
    pub struct DOMDOMSelection(Object<ffi::WebKitDOMDOMSelection>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_selection_get_type(),
    }
}

impl DOMDOMSelection {
    #[cfg(feature = "v2_16")]
    pub fn add_range(&self, range: &DOMRange) {
        unsafe {
            ffi::webkit_dom_dom_selection_add_range(self.to_glib_none().0, range.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn collapse<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_collapse(self.to_glib_none().0, node.to_glib_none().0, offset);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn collapse_to_end(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_end(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn collapse_to_start(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_start(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn contains_node<P: IsA<DOMNode>>(&self, node: &P, allowPartial: bool) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_contains_node(self.to_glib_none().0, node.to_glib_none().0, allowPartial.to_glib()))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn delete_from_document(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_delete_from_document(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn empty(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_empty(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn extend<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_extend(self.to_glib_none().0, node.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_anchor_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_anchor_node(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_anchor_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_anchor_offset(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_base_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_base_node(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_base_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_base_offset(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_extent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_extent_node(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_extent_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_extent_offset(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_focus_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_focus_node(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_focus_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_focus_offset(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_is_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_get_is_collapsed(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_range_at(&self, index: libc::c_ulong) -> Result<DOMRange, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_selection_get_range_at(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_range_count(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_range_count(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn get_selection_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_selection_get_selection_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn modify(&self, alter: &str, direction: &str, granularity: &str) {
        unsafe {
            ffi::webkit_dom_dom_selection_modify(self.to_glib_none().0, alter.to_glib_none().0, direction.to_glib_none().0, granularity.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn remove_all_ranges(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_remove_all_ranges(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn select_all_children<P: IsA<DOMNode>>(&self, node: &P) {
        unsafe {
            ffi::webkit_dom_dom_selection_select_all_children(self.to_glib_none().0, node.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn set_base_and_extent<P: IsA<DOMNode>, Q: IsA<DOMNode>>(&self, baseNode: &P, baseOffset: libc::c_ulong, extentNode: &Q, extentOffset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_base_and_extent(self.to_glib_none().0, baseNode.to_glib_none().0, baseOffset, extentNode.to_glib_none().0, extentOffset);
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn set_position<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_position(self.to_glib_none().0, node.to_glib_none().0, offset);
        }
    }

    pub fn get_property_anchor_node(&self) -> Option<DOMNode> {
        let mut value = Value::from(None::<&DOMNode>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "anchor-node".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_base_node(&self) -> Option<DOMNode> {
        let mut value = Value::from(None::<&DOMNode>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "base-node".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_extent_node(&self) -> Option<DOMNode> {
        let mut value = Value::from(None::<&DOMNode>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "extent-node".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_focus_node(&self) -> Option<DOMNode> {
        let mut value = Value::from(None::<&DOMNode>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "focus-node".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_is_collapsed(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-collapsed".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
