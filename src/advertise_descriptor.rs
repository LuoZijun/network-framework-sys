use crate::*;

use libc::c_char;
use libc::c_void;


pub enum nw_advertise_descriptor {}
pub type nw_advertise_descriptor_t = *mut nw_advertise_descriptor;


extern "C" {
    pub fn nw_advertise_descriptor_create_bonjour_service(name: *const c_char,
                                                          type_: *const c_char,
                                                          domain: *const c_char)
        -> nw_advertise_descriptor_t;
    pub fn nw_advertise_descriptor_set_txt_record(advertise_descriptor: nw_advertise_descriptor_t,
                                                  txt_record: *const c_void,
                                                  txt_length: isize);
    pub fn nw_advertise_descriptor_set_no_auto_rename(advertise_descriptor: nw_advertise_descriptor_t,
                                                      no_auto_rename: bool);
    pub fn nw_advertise_descriptor_get_no_auto_rename(advertise_descriptor: nw_advertise_descriptor_t) -> bool;
}