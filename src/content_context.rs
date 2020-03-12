use crate::*;

use libc::c_char;
use libc::c_double;


pub enum nw_content_context {}
pub type nw_content_context_t = *mut nw_content_context;


extern "C" {
    pub fn nw_content_context_create(context_identifier: *const c_char) -> nw_content_context_t;
    pub fn nw_content_context_get_identifier(context: nw_content_context_t) -> *const c_char;

    pub fn nw_content_context_get_is_final(context: nw_content_context_t) -> bool;
    pub fn nw_content_context_set_is_final(context: nw_content_context_t, is_final: bool);
    pub fn nw_content_context_get_expiration_milliseconds(context: nw_content_context_t) -> u64;
    pub fn nw_content_context_set_expiration_milliseconds(context: nw_content_context_t, expiration_milliseconds: u64);

    pub fn nw_content_context_get_relative_priority(context: nw_content_context_t) -> c_double;
    pub fn nw_content_context_set_relative_priority(context: nw_content_context_t, relative_priority: c_double);
    pub fn nw_content_context_set_antecedent(context: nw_content_context_t, antecendent_context: nw_content_context_t);
    pub fn nw_content_context_copy_antecedent(context: nw_content_context_t) -> nw_content_context_t;
    pub fn nw_content_context_set_metadata_for_protocol(context: nw_content_context_t, protocol_metadata: nw_protocol_metadata_t);
    pub fn nw_content_context_copy_protocol_metadata(context: nw_content_context_t,
                                                     protocol: nw_protocol_definition_t)
        -> nw_protocol_metadata_t;

    // void (^foreach_block)(nw_protocol_definition_t definition, nw_protocol_metadata_t metadata)
    // pub fn nw_content_context_foreach_protocol_metadata(context: nw_content_context_t,
    //                                                     foreach_block: foreach_block_t);
}