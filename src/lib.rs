#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

extern crate libc;
extern crate core_foundation_sys;


#[link(name = "Network", kind = "framework")]
extern "C" {}

// Dispatch.Framework
// https://developer.apple.com/documentation/dispatch/dispatch_queue_t
pub enum dispatch_queue { }
pub type dispatch_queue_t = *mut dispatch_queue;

// Dispatch.Framework
// https://developer.apple.com/documentation/dispatch/dispatch_block_t?language=objc
// typedef void (^dispatch_block_t)(void);
pub type dispatch_block_t = extern "C" fn();

// Security.Framework
// https://developer.apple.com/documentation/security/sec_protocol_options_t?language=objc
pub enum sec_protocol_options { }
pub type sec_protocol_options_t = *mut sec_protocol_options;

// Security.Framework
// https://developer.apple.com/documentation/security/sec_protocol_metadata_t
pub enum sec_protocol_metadata { }
pub type sec_protocol_metadata_t = *mut sec_protocol_metadata;


mod advertise_descriptor;
mod endpoint;
mod ip_options;
mod nw_object;
mod path_monitor;
mod tls_options;
mod connection;
mod error;
mod parameters;
mod protocol_options;
mod udp_options;
mod content_context;
mod interface;
mod listener;
mod path;
mod tcp_options;

pub use self::advertise_descriptor::*;
pub use self::endpoint::*;
pub use self::ip_options::*;
pub use self::nw_object::*;
pub use self::path_monitor::*;
pub use self::tls_options::*;
pub use self::connection::*;
pub use self::error::*;
pub use self::parameters::*;
pub use self::protocol_options::*;
pub use self::udp_options::*;
pub use self::content_context::*;
pub use self::interface::*;
pub use self::listener::*;
pub use self::path::*;
pub use self::tcp_options::*;


#[test]
fn test_crate_tls_options() {
    unsafe {
        let tls_opts = nw_tls_create_options();
        assert!(!tls_opts.is_null());
    }
}
