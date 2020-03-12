use crate::*;

extern "C" {
    pub fn nw_protocol_copy_udp_definition() -> nw_protocol_definition_t;
    pub fn nw_udp_create_options() -> nw_protocol_options_t;
    pub fn nw_udp_options_set_prefer_no_checksum(options: nw_protocol_options_t,
                                                 prefer_no_checksum: bool);
    pub fn nw_udp_create_metadata() -> nw_protocol_metadata_t;
    pub fn nw_protocol_metadata_is_udp(metadata: nw_protocol_metadata_t) -> bool;
}