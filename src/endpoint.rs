use crate::*;

use libc::c_int;
use libc::c_char;
use libc::sockaddr;


pub enum nw_endpoint {}
pub type nw_endpoint_t = *mut nw_endpoint;

pub type nw_endpoint_type_t = c_int;
pub const nw_endpoint_type_invalid: nw_endpoint_type_t         = 0;
pub const nw_endpoint_type_address: nw_endpoint_type_t         = 1;
pub const nw_endpoint_type_host: nw_endpoint_type_t            = 2;
pub const nw_endpoint_type_bonjour_service: nw_endpoint_type_t = 3;


extern "C" {
    pub fn nw_endpoint_get_type(endpoint: nw_endpoint_t) -> nw_endpoint_type_t;
    pub fn nw_endpoint_create_host(hostname: *const c_char, port: *const c_char) -> nw_endpoint_t;
    pub fn nw_endpoint_get_hostname(endpoint: nw_endpoint_t) -> *const c_char;
    pub fn nw_endpoint_copy_port_string(endpoint: nw_endpoint_t) -> *mut c_char;
    pub fn nw_endpoint_get_port(endpoint: nw_endpoint_t) -> u16;
    pub fn nw_endpoint_create_address(address: *const sockaddr) -> nw_endpoint_t;
    pub fn nw_endpoint_copy_address_string(endpoint: nw_endpoint_t) -> *mut c_char;
    pub fn nw_endpoint_get_address(endpoint: nw_endpoint_t) -> *const sockaddr;

    pub fn nw_endpoint_create_bonjour_service(name: *const c_char,
                                              type_: *const c_char,
                                              domain: *const c_char) -> nw_endpoint_t;
    pub fn nw_endpoint_get_bonjour_service_name(endpoint: nw_endpoint_t) -> *const c_char;
    pub fn nw_endpoint_get_bonjour_service_type(endpoint: nw_endpoint_t) -> *const c_char;
    pub fn nw_endpoint_get_bonjour_service_domain(endpoint: nw_endpoint_t) -> *const c_char;
}

