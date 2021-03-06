// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

use PlayerStreamInfo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerVideoInfo(Object<ffi::GstPlayerVideoInfo>): PlayerStreamInfo;

    match fn {
        get_type => || ffi::gst_player_video_info_get_type(),
    }
}

unsafe impl Send for PlayerVideoInfo {}
unsafe impl Sync for PlayerVideoInfo {}

pub trait PlayerVideoInfoExt {
    fn get_bitrate(&self) -> i32;

    fn get_framerate(&self) -> (i32, i32);

    fn get_height(&self) -> i32;

    fn get_max_bitrate(&self) -> i32;

    fn get_pixel_aspect_ratio(&self) -> (u32, u32);

    fn get_width(&self) -> i32;
}

impl<O: IsA<PlayerVideoInfo>> PlayerVideoInfoExt for O {
    fn get_bitrate(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_bitrate(self.to_glib_none().0)
        }
    }

    fn get_framerate(&self) -> (i32, i32) {
        unsafe {
            let mut fps_n = mem::uninitialized();
            let mut fps_d = mem::uninitialized();
            ffi::gst_player_video_info_get_framerate(self.to_glib_none().0, &mut fps_n, &mut fps_d);
            (fps_n, fps_d)
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_height(self.to_glib_none().0)
        }
    }

    fn get_max_bitrate(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_max_bitrate(self.to_glib_none().0)
        }
    }

    fn get_pixel_aspect_ratio(&self) -> (u32, u32) {
        unsafe {
            let mut par_n = mem::uninitialized();
            let mut par_d = mem::uninitialized();
            ffi::gst_player_video_info_get_pixel_aspect_ratio(self.to_glib_none().0, &mut par_n, &mut par_d);
            (par_n, par_d)
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gst_player_video_info_get_width(self.to_glib_none().0)
        }
    }
}
