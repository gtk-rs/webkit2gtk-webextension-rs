// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use DOMEventTarget;
use DOMObject;
use Object;
use ffi;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use libc;

glib_wrapper! {
    pub struct DOMEvent(Object<ffi::WebKitDOMEvent>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_event_get_type(),
    }
}

pub trait DOMEventExt {
    fn get_bubbles(&self) -> bool;

    fn get_cancel_bubble(&self) -> bool;

    fn get_cancelable(&self) -> bool;

    fn get_current_target(&self) -> Option<DOMEventTarget>;

    fn get_event_phase(&self) -> libc::c_ushort;

    fn get_event_type(&self) -> Option<String>;

    fn get_return_value(&self) -> bool;

    fn get_src_element(&self) -> Option<DOMEventTarget>;

    fn get_target(&self) -> Option<DOMEventTarget>;

    fn get_time_stamp(&self) -> u32;

    fn init_event(&self, eventTypeArg: &str, canBubbleArg: bool, cancelableArg: bool);

    fn prevent_default(&self);

    fn set_cancel_bubble(&self, value: bool);

    fn set_return_value(&self, value: bool);

    fn stop_propagation(&self);

    fn get_property_type(&self) -> Option<String>;
}

impl<O: IsA<DOMEvent> + IsA<Object>> DOMEventExt for O {
    fn get_bubbles(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_bubbles(self.to_glib_none().0))
        }
    }

    fn get_cancel_bubble(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_cancel_bubble(self.to_glib_none().0))
        }
    }

    fn get_cancelable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_cancelable(self.to_glib_none().0))
        }
    }

    fn get_current_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_current_target(self.to_glib_none().0))
        }
    }

    fn get_event_phase(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_event_get_event_phase(self.to_glib_none().0)
        }
    }

    fn get_event_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_event_type(self.to_glib_none().0))
        }
    }

    fn get_return_value(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_return_value(self.to_glib_none().0))
        }
    }

    fn get_src_element(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_src_element(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_target(self.to_glib_none().0))
        }
    }

    fn get_time_stamp(&self) -> u32 {
        unsafe {
            ffi::webkit_dom_event_get_time_stamp(self.to_glib_none().0)
        }
    }

    fn init_event(&self, eventTypeArg: &str, canBubbleArg: bool, cancelableArg: bool) {
        unsafe {
            ffi::webkit_dom_event_init_event(self.to_glib_none().0, eventTypeArg.to_glib_none().0, canBubbleArg.to_glib(), cancelableArg.to_glib());
        }
    }

    fn prevent_default(&self) {
        unsafe {
            ffi::webkit_dom_event_prevent_default(self.to_glib_none().0);
        }
    }

    fn set_cancel_bubble(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_event_set_cancel_bubble(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_return_value(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_event_set_return_value(self.to_glib_none().0, value.to_glib());
        }
    }

    fn stop_propagation(&self) {
        unsafe {
            ffi::webkit_dom_event_stop_propagation(self.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
