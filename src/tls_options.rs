use crate::*;

extern "C" {
    pub fn nw_protocol_copy_tls_definition() -> nw_protocol_definition_t;
    pub fn nw_tls_create_options() -> nw_protocol_options_t;
    pub fn nw_tls_copy_sec_protocol_options() -> sec_protocol_options_t;
    pub fn nw_protocol_metadata_is_tls(metadata: nw_protocol_metadata_t)
        -> bool;
    pub fn nw_tls_copy_sec_protocol_metadata(metadata: nw_protocol_metadata_t)
        -> sec_protocol_metadata_t;
}