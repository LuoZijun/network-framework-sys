use crate::*;

use libc::c_int;


pub enum nw_path {}
pub type nw_path_t = *mut nw_path;

pub type nw_path_status_t = c_int;
pub const nw_path_status_invalid: nw_path_status_t     = 0;
pub const nw_path_status_satisfied: nw_path_status_t   = 1;
pub const nw_path_status_unsatisfied: nw_path_status_t = 2;
pub const nw_path_status_satisfiable: nw_path_status_t = 3;


extern "C" {
    pub fn nw_path_get_status(path: nw_path_t) -> nw_path_status_t;
    
    // typedef bool (^nw_path_enumerate_interfaces_block_t)(nw_interface_t interface);
    // pub fn nw_path_enumerate_interfaces(path: nw_path_t,
    //                                     enumerate_block: nw_path_enumerate_interfaces_block_t);

    pub fn nw_path_is_equal(path: nw_path_t, other_path: nw_path_t) -> bool;
    pub fn nw_path_is_expensive(path: nw_path_t) -> bool;
    pub fn nw_path_has_ipv4(path: nw_path_t) -> bool;
    pub fn nw_path_has_ipv6(path: nw_path_t) -> bool;
    pub fn nw_path_has_dns(path: nw_path_t) -> bool;
    pub fn nw_path_uses_interface_type(path: nw_path_t,
                                       interface_type: nw_interface_type_t) -> bool;
    pub fn nw_path_copy_effective_local_endpoint(path: nw_path_t) -> nw_endpoint_t;
    pub fn nw_path_copy_effective_remote_endpoint(path: nw_path_t) -> nw_endpoint_t;
}