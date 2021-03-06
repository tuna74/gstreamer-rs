// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

use Iterator;
use Object;
use Pad;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ProxyPad(Object<ffi::GstProxyPad>): Pad, Object;

    match fn {
        get_type => || ffi::gst_proxy_pad_get_type(),
    }
}

impl ProxyPad {
    //pub fn chain_list_default<'a, P: IsA<Pad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(pad: &P, parent: R, list: /*Ignored*/&mut BufferList) -> FlowReturn {
    //    unsafe { TODO: call ffi::gst_proxy_pad_chain_list_default() }
    //}

    pub fn iterate_internal_links_default<'a, P: IsA<Pad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(pad: &P, parent: R) -> Option<Iterator> {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_iterate_internal_links_default(pad.to_glib_none().0, parent.0))
        }
    }
}

unsafe impl Send for ProxyPad {}
unsafe impl Sync for ProxyPad {}

pub trait ProxyPadExt {
    fn get_internal(&self) -> Option<ProxyPad>;
}

impl<O: IsA<ProxyPad>> ProxyPadExt for O {
    fn get_internal(&self) -> Option<ProxyPad> {
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_get_internal(self.to_glib_none().0))
        }
    }
}
