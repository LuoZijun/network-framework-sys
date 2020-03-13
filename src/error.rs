use crate::*;

use libc::c_int;
use core_foundation_sys::error::CFErrorRef;
use core_foundation_sys::string::CFStringRef;


pub enum nw_error {}
pub type nw_error_t = *mut nw_error;

pub type nw_error_domain_t = c_int;
pub const nw_error_domain_invalid: nw_error_domain_t = 0;
pub const nw_error_domain_posix: nw_error_domain_t   = 1;
pub const nw_error_domain_dns: nw_error_domain_t     = 2;
pub const nw_error_domain_tls: nw_error_domain_t     = 3;


extern "C" {
    pub static kNWErrorDomainPOSIX: CFStringRef;
    pub static kNWErrorDomainDNS: CFStringRef;
    pub static kNWErrorDomainTLS: CFStringRef;
    
    pub fn nw_error_get_error_domain(error: nw_error_t) -> nw_error_domain_t;
    pub fn nw_error_get_error_code(error: nw_error_t) -> c_int;
    pub fn nw_error_copy_cf_error(error: nw_error_t) -> CFErrorRef;
    
}