// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v2_16")]
use DOMDocumentFragment;
use DOMNode;
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
    pub struct DOMRange(Object<ffi::WebKitDOMRange>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_range_get_type(),
    }
}

pub trait DOMRangeExt {
    #[cfg(feature = "v2_16")]
    fn clone_contents(&self) -> Result<DOMDocumentFragment, Error>;

    fn clone_range(&self) -> Result<DOMRange, Error>;

    fn collapse(&self, toStart: bool) -> Result<(), Error>;

    fn compare_boundary_points(&self, how: libc::c_ushort, sourceRange: &DOMRange) -> Result<libc::c_short, Error>;

    fn compare_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<libc::c_short, Error>;

    fn compare_point<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<libc::c_short, Error>;

    #[cfg(feature = "v2_16")]
    fn create_contextual_fragment(&self, html: &str) -> Result<DOMDocumentFragment, Error>;

    fn delete_contents(&self) -> Result<(), Error>;

    fn detach(&self) -> Result<(), Error>;

    #[cfg(feature = "v2_16")]
    fn expand(&self, unit: &str) -> Result<(), Error>;

    #[cfg(feature = "v2_16")]
    fn extract_contents(&self) -> Result<DOMDocumentFragment, Error>;

    fn get_collapsed(&self) -> Result<(), Error>;

    fn get_common_ancestor_container(&self) -> Result<DOMNode, Error>;

    fn get_end_container(&self) -> Result<DOMNode, Error>;

    fn get_end_offset(&self) -> Result<libc::c_long, Error>;

    fn get_start_container(&self) -> Result<DOMNode, Error>;

    fn get_start_offset(&self) -> Result<libc::c_long, Error>;

    fn get_text(&self) -> Option<String>;

    fn insert_node<P: IsA<DOMNode>>(&self, newNode: &P) -> Result<(), Error>;

    fn intersects_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn is_point_in_range<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    fn select_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn select_node_contents<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn set_end<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    fn set_end_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn set_end_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn set_start<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error>;

    fn set_start_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn set_start_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error>;

    fn surround_contents<P: IsA<DOMNode>>(&self, newParent: &P) -> Result<(), Error>;

    fn to_string(&self) -> Result<String, Error>;

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_common_ancestor_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_end_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_end_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_start_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_start_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMRange> + IsA<glib::object::Object>> DOMRangeExt for O {
    #[cfg(feature = "v2_16")]
    fn clone_contents(&self) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_clone_contents(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn clone_range(&self) -> Result<DOMRange, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_clone_range(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn collapse(&self, toStart: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_collapse(self.to_glib_none().0, toStart.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_boundary_points(&self, how: libc::c_ushort, sourceRange: &DOMRange) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_boundary_points(self.to_glib_none().0, how, sourceRange.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_node(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn compare_point<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<libc::c_short, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_compare_point(self.to_glib_none().0, refNode.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    fn create_contextual_fragment(&self, html: &str) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_create_contextual_fragment(self.to_glib_none().0, html.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_contents(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_delete_contents(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn detach(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_detach(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    fn expand(&self, unit: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_expand(self.to_glib_none().0, unit.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_16")]
    fn extract_contents(&self) -> Result<DOMDocumentFragment, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_extract_contents(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_collapsed(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_get_collapsed(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_common_ancestor_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_common_ancestor_container(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_end_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_end_container(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_end_offset(&self) -> Result<libc::c_long, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_end_offset(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_start_container(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_start_container(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_start_offset(&self) -> Result<libc::c_long, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_get_start_offset(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_range_get_text(self.to_glib_none().0))
        }
    }

    fn insert_node<P: IsA<DOMNode>>(&self, newNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_insert_node(self.to_glib_none().0, newNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn intersects_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_intersects_node(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_point_in_range<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_is_point_in_range(self.to_glib_none().0, refNode.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_node<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_select_node(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_node_contents<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_select_node_contents(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end(self.to_glib_none().0, refNode.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end_after(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_end_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_end_before(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start<P: IsA<DOMNode>>(&self, refNode: &P, offset: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start(self.to_glib_none().0, refNode.to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start_after<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start_after(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_start_before<P: IsA<DOMNode>>(&self, refNode: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_set_start_before(self.to_glib_none().0, refNode.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn surround_contents<P: IsA<DOMNode>>(&self, newParent: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_range_surround_contents(self.to_glib_none().0, newParent.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn to_string(&self) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_range_to_string(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::collapsed",
                transmute(notify_collapsed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_common_ancestor_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::common-ancestor-container",
                transmute(notify_common_ancestor_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_end_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::end-container",
                transmute(notify_end_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_end_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::end-offset",
                transmute(notify_end_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_container_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::start-container",
                transmute(notify_start_container_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::start-offset",
                transmute(notify_start_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_collapsed_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_common_ancestor_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_end_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_end_offset_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_start_container_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_start_offset_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMRange, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMRange> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMRange::from_glib_borrow(this).downcast_unchecked())
}
