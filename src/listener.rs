use crate::*;

use libc::c_int;
use libc::c_char;


pub enum nw_listener {}
pub type nw_listener_t = *mut nw_listener;

pub type nw_listener_state_t = c_int;
pub const nw_listener_state_invalid: nw_listener_state_t   = 0;
pub const nw_listener_state_waiting: nw_listener_state_t   = 1;
pub const nw_listener_state_ready: nw_listener_state_t     = 2;
pub const nw_listener_state_failed: nw_listener_state_t    = 3;
pub const nw_listener_state_cancelled: nw_listener_state_t = 4;


extern "C" {
    pub fn nw_listener_create_with_port(port: *const c_char,
                                        parameters: nw_parameters_t)
        -> nw_listener_t;
    pub fn nw_listener_create(parameters: nw_parameters_t) -> nw_listener_t;
    pub fn nw_listener_create_with_connection(connection: nw_connection_t,
                                              parameters: nw_parameters_t)
        -> nw_listener_t;
    pub fn nw_listener_set_queue(listener: nw_listener_t, queue: dispatch_queue_t);


    // typedef void (^nw_listener_state_changed_handler_t)(nw_listener_state_t state, _Nullable nw_error_t error);
    // pub fn nw_listener_set_state_changed_handler(listener: nw_listener_t,
    //                                              handler: nw_listener_state_changed_handler_t);

    // typedef void (^nw_listener_new_connection_handler_t)(nw_connection_t connection);
    // pub fn nw_listener_set_new_connection_handler(listener: nw_listener_t,
    //                                               handler: nw_listener_new_connection_handler_t);

    pub fn nw_listener_set_advertise_descriptor(listener: nw_listener_t,
                                                advertise_descriptor: nw_advertise_descriptor_t);

    // typedef void (^nw_listener_advertised_endpoint_changed_handler_t)(nw_endpoint_t advertised_endpoint, bool added);
    // pub fn nw_listener_set_advertised_endpoint_changed_handler(listener: nw_listener_t,
    //                                                            handler: nw_listener_advertised_endpoint_changed_handler_t);

    pub fn nw_listener_get_port(listener: nw_listener_t) -> u16;
    pub fn nw_listener_start(listener: nw_listener_t);
    pub fn nw_listener_cancel(listener: nw_listener_t);
}
