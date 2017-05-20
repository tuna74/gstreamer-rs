// This file was generated by gir (8ed6d6b+) from gir-files (???)
// DO NOT EDIT

use Element;
use ElementFactoryListType;
use Object;
use URIType;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ElementFactory(Object<ffi::GstElementFactory>): Object;

    match fn {
        get_type => || ffi::gst_element_factory_get_type(),
    }
}

impl ElementFactory {
    pub fn find(name: &str) -> Option<ElementFactory> {
        unsafe {
            from_glib_full(ffi::gst_element_factory_find(name.to_glib_none().0))
        }
    }

    //pub fn list_filter(list: &[ElementFactory], caps: /*Ignored*/&Caps, direction: PadDirection, subsetonly: bool) -> Vec<ElementFactory> {
    //    unsafe { TODO: call ffi::gst_element_factory_list_filter() }
    //}

    //pub fn list_get_elements(type_: ElementFactoryListType, minrank: /*Ignored*/Rank) -> Vec<ElementFactory> {
    //    unsafe { TODO: call ffi::gst_element_factory_list_get_elements() }
    //}

    pub fn make<'a, P: Into<Option<&'a str>>>(factoryname: &str, name: P) -> Option<Element> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_element_factory_make(factoryname.to_glib_none().0, name.0))
        }
    }
}

pub trait ElementFactoryExt {
    //fn can_sink_all_caps(&self, caps: /*Ignored*/&Caps) -> bool;

    //fn can_sink_any_caps(&self, caps: /*Ignored*/&Caps) -> bool;

    //fn can_src_all_caps(&self, caps: /*Ignored*/&Caps) -> bool;

    //fn can_src_any_caps(&self, caps: /*Ignored*/&Caps) -> bool;

    fn create<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<Element>;

    fn get_element_type(&self) -> glib::types::Type;

    fn get_metadata(&self, key: &str) -> Option<String>;

    fn get_metadata_keys(&self) -> Vec<String>;

    fn get_num_pad_templates(&self) -> u32;

    //fn get_static_pad_templates(&self) -> /*Ignored*/Vec<StaticPadTemplate>;

    fn get_uri_protocols(&self) -> Vec<String>;

    fn get_uri_type(&self) -> URIType;

    fn has_interface(&self, interfacename: &str) -> bool;

    fn list_is_type(&self, type_: ElementFactoryListType) -> bool;
}

impl<O: IsA<ElementFactory>> ElementFactoryExt for O {
    //fn can_sink_all_caps(&self, caps: /*Ignored*/&Caps) -> bool {
    //    unsafe { TODO: call ffi::gst_element_factory_can_sink_all_caps() }
    //}

    //fn can_sink_any_caps(&self, caps: /*Ignored*/&Caps) -> bool {
    //    unsafe { TODO: call ffi::gst_element_factory_can_sink_any_caps() }
    //}

    //fn can_src_all_caps(&self, caps: /*Ignored*/&Caps) -> bool {
    //    unsafe { TODO: call ffi::gst_element_factory_can_src_all_caps() }
    //}

    //fn can_src_any_caps(&self, caps: /*Ignored*/&Caps) -> bool {
    //    unsafe { TODO: call ffi::gst_element_factory_can_src_any_caps() }
    //}

    fn create<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<Element> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_element_factory_create(self.to_glib_none().0, name.0))
        }
    }

    fn get_element_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_element_factory_get_element_type(self.to_glib_none().0))
        }
    }

    fn get_metadata(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_element_factory_get_metadata(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_metadata_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_get_metadata_keys(self.to_glib_none().0))
        }
    }

    fn get_num_pad_templates(&self) -> u32 {
        unsafe {
            ffi::gst_element_factory_get_num_pad_templates(self.to_glib_none().0)
        }
    }

    //fn get_static_pad_templates(&self) -> /*Ignored*/Vec<StaticPadTemplate> {
    //    unsafe { TODO: call ffi::gst_element_factory_get_static_pad_templates() }
    //}

    fn get_uri_protocols(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_element_factory_get_uri_protocols(self.to_glib_none().0))
        }
    }

    fn get_uri_type(&self) -> URIType {
        unsafe {
            from_glib(ffi::gst_element_factory_get_uri_type(self.to_glib_none().0))
        }
    }

    fn has_interface(&self, interfacename: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_has_interface(self.to_glib_none().0, interfacename.to_glib_none().0))
        }
    }

    fn list_is_type(&self, type_: ElementFactoryListType) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_list_is_type(self.to_glib_none().0, type_))
        }
    }
}
