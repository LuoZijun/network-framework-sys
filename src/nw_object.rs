use crate::*;

use libc::c_void;


pub enum nw_object {}
pub type nw_object_t = *mut nw_object;


extern "C" {
    pub fn nw_retain(obj: *mut c_void) -> *mut c_void;
    pub fn nw_release(obj: *mut c_void);
}