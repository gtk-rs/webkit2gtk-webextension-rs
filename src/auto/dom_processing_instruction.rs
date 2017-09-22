// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use DOMStyleSheet;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMProcessingInstruction(Object<ffi::WebKitDOMProcessingInstruction>): DOMCharacterData, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_processing_instruction_get_type(),
    }
}

pub trait DOMProcessingInstructionExt {
    fn get_sheet(&self) -> Option<DOMStyleSheet>;

    fn get_target(&self) -> Option<String>;

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMProcessingInstruction> + IsA<glib::object::Object>> DOMProcessingInstructionExt for O {
    fn get_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_processing_instruction_get_sheet(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_processing_instruction_get_target(self.to_glib_none().0))
        }
    }

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sheet",
                transmute(notify_sheet_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_sheet_trampoline<P>(this: *mut ffi::WebKitDOMProcessingInstruction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMProcessingInstruction> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMProcessingInstruction::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMProcessingInstruction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMProcessingInstruction> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMProcessingInstruction::from_glib_borrow(this).downcast_unchecked())
}
