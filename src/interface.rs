use crate::*;

use libc::c_int;
use libc::c_char;


pub enum nw_interface {}
pub type nw_interface_t = *mut nw_interface;

pub type nw_interface_type_t = c_int;
pub const nw_interface_type_other: nw_interface_type_t    = 0;
pub const nw_interface_type_wifi: nw_interface_type_t     = 1;
pub const nw_interface_type_cellular: nw_interface_type_t = 2;
pub const nw_interface_type_wired: nw_interface_type_t    = 3;
pub const nw_interface_type_loopback: nw_interface_type_t = 4;


extern "C" {
    pub fn nw_interface_get_type(interface: nw_interface_t) -> nw_interface_type_t;
    pub fn nw_interface_get_name(interface: nw_interface_t) -> *const c_char;
    pub fn nw_interface_get_index(interface: nw_interface_t) -> u32;
}