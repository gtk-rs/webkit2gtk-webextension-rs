// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
use DOMDocumentFragment;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMRange(Object<ffi::WebKitDOMRange, ffi::WebKitDOMRangeClass, DOMRangeClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_range_get_type(),
    }
}

pub const NONE_DOM_RANGE: Option<&DOMRange> = None;

pub trait DOMRangeExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn clone_contents(&self) -> Result<DOMDocumentFragment, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn clone_range(&self) -> Result<DOMRange, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn collapse(&self, toStart: bool) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn compare_boundary_points<P: IsA<DOMRange>>(&self, how: libc::c_ushort, sourceRange: &P) -> Result<libc::c_short, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn compare_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<libc::c_short, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn compare_point<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<libc::c_short, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn create_contextual_fragment(&self, html: &str) -> Result<DOMDocumentFragment, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_contents(&self) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn detach(&self) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn expand(&self, unit: &str) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn extract_contents(&self) -> Result<DOMDocumentFragment, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_collapsed(&self) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_common_ancestor_container(&self) -> Result<DOMNode, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_end_container(&self) -> Result<DOMNode, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_end_offset(&self) -> Result<libc::c_long, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_start_container(&self) -> Result<DOMNode, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_start_offset(&self) -> Result<libc::c_long, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn insert_node<P: IsA<DOMNode>>(&self, newNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn intersects_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn is_point_in_range<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn select_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn select_node_contents<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_end<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_end_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_end_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_start<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_start_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_start_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn surround_contents<P: IsA<DOMNode>>(&self, newParent: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn to_string(&self) -> Result<GString, Error>;

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_common_ancestor_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_end_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_end_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMRange>> DOMRangeExt for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn clone_contents(&self) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_clone_contents(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn clone_range(&self) -> Result<DOMRange, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_clone_range(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn collapse(&self, toStart: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_collapse(self.as_ref().to_glib_none().0, toStart.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_boundary_points<P: IsA<DOMRange>>(&self, how: libc::c_ushort, sourceRange: &P) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_boundary_points(self.as_ref().to_glib_none().0, how, sourceRange.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_node(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_point<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_point(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn create_contextual_fragment(&self, html: &str) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_create_contextual_fragment(self.as_ref().to_glib_none().0, html.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_contents(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_delete_contents(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn detach(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_detach(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn expand(&self, unit: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_expand(self.as_ref().to_glib_none().0, unit.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn extract_contents(&self) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_extract_contents(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_collapsed(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_get_collapsed(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_common_ancestor_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_common_ancestor_container(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_end_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_end_container(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_end_offset(&self) -> Result<libc::c_long, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_end_offset(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_start_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_start_container(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_start_offset(&self) -> Result<libc::c_long, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_start_offset(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_range_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_node<P: IsA<DOMNode>>(&self, newNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_insert_node(self.as_ref().to_glib_none().0, newNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn intersects_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_intersects_node(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_point_in_range<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_is_point_in_range(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_select_node(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_node_contents<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_select_node_contents(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end_after(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end_before(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start_after(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start_before(self.as_ref().to_glib_none().0, refNode.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn surround_contents<P: IsA<DOMNode>>(&self, newParent: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_surround_contents(self.as_ref().to_glib_none().0, newParent.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn to_string(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_to_string(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::collapsed\0".as_ptr() as *const _,
                transmute(notify_collapsed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_common_ancestor_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::common-ancestor-container\0".as_ptr() as *const _,
                transmute(notify_common_ancestor_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_end_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::end-container\0".as_ptr() as *const _,
                transmute(notify_end_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_end_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::end-offset\0".as_ptr() as *const _,
                transmute(notify_end_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::start-container\0".as_ptr() as *const _,
                transmute(notify_start_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::start-offset\0".as_ptr() as *const _,
                transmute(notify_start_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_collapsed_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_common_ancestor_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_end_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_end_offset_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_start_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_start_offset_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMRange")
    }
}
