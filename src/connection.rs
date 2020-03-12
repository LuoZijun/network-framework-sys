use crate::*;

use libc::c_int;
use libc::c_char;


pub enum nw_connection {}
pub type nw_connection_t = *mut nw_connection;

pub type nw_connection_state_t = c_int;
pub const nw_connection_state_invalid: nw_connection_state_t   = 0;
pub const nw_connection_state_waiting: nw_connection_state_t   = 1;
pub const nw_connection_state_preparing: nw_connection_state_t = 2;
pub const nw_connection_state_ready: nw_connection_state_t     = 3;
pub const nw_connection_state_failed: nw_connection_state_t    = 4;
pub const nw_connection_state_cancelled: nw_connection_state_t = 5;


extern "C" {
    pub fn nw_connection_create(endpoint: nw_endpoint_t, parameters: nw_parameters_t) -> nw_connection_t;
    pub fn nw_connection_copy_endpoint(connection: nw_connection_t) -> nw_endpoint_t;
    pub fn nw_connection_copy_parameters(connection: nw_connection_t) -> nw_parameters_t;

    pub fn nw_connection_set_queue(connection: nw_connection_t, queue: dispatch_queue_t);
    pub fn nw_connection_start(connection: nw_connection_t);
    pub fn nw_connection_restart(connection: nw_connection_t);
    pub fn nw_connection_cancel(connection: nw_connection_t);
    pub fn nw_connection_force_cancel(connection: nw_connection_t);
    pub fn nw_connection_cancel_current_endpoint(connection: nw_connection_t);

    // typedef void (^nw_connection_receive_completion_t)(_Nullable dispatch_data_t content,
    //                                                _Nullable nw_content_context_t context,
    //                                                bool is_complete,
    //                                                _Nullable nw_error_t error);
    // pub fn nw_connection_receive(connection: nw_connection_t,
    //                              minimum_incomplete_length: u32,
    //                              maximum_length: u32,
    //                              completion: nw_connection_receive_completion_t);
    // pub fn nw_connection_receive_message(connection: nw_connection_t,
    //                                      completion: nw_connection_receive_completion_t);

    // typedef void (^nw_connection_send_completion_t)(_Nullable nw_error_t error);
    // pub fn nw_connection_send(connection: nw_connection_t,
    //                           content: dispatch_data_t,
    //                           context: nw_content_context_t,
    //                           is_complete: bool,
    //                           completion: nw_connection_send_completion_t);
    pub fn nw_connection_batch(connection: nw_connection_t, batch_block: dispatch_block_t);
    pub fn nw_connection_copy_description(connection: nw_connection_t) -> *mut c_char;
    pub fn nw_connection_copy_current_path(connection: nw_connection_t) -> nw_path_t;

    pub fn nw_connection_copy_protocol_metadata(connection: nw_connection_t,
                                                definition: nw_protocol_definition_t)
        -> nw_protocol_metadata_t;
    pub fn nw_connection_get_maximum_datagram_size(connection: nw_connection_t) -> u32;
}

