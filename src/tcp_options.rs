use crate::*;

extern "C" {
    pub fn nw_protocol_copy_tcp_definition() -> nw_protocol_definition_t;
    pub fn nw_tcp_create_options() -> nw_protocol_options_t;
    pub fn nw_tcp_options_set_no_delay(options: nw_protocol_options_t,
                                       no_delay: bool);
    pub fn nw_tcp_options_set_no_push(options: nw_protocol_options_t,
                                      no_push: bool);
    pub fn nw_tcp_options_set_no_options(options: nw_protocol_options_t,
                                         no_options: bool);
    pub fn nw_tcp_options_set_enable_keepalive(options: nw_protocol_options_t,
                                               enable_keepalive: bool);
    pub fn nw_tcp_options_set_keepalive_count(options: nw_protocol_options_t,
                                              keepalive_count: u32);
    pub fn nw_tcp_options_set_keepalive_idle_time(options: nw_protocol_options_t,
                                                  keepalive_idle_time: u32);
    pub fn nw_tcp_options_set_keepalive_interval(options: nw_protocol_options_t,
                                                 keepalive_interval: u32);
    pub fn nw_tcp_options_set_maximum_segment_size(options: nw_protocol_options_t,
                                                   maximum_segment_size: u32);
    pub fn nw_tcp_options_set_connection_timeout(options: nw_protocol_options_t,
                                                 connection_timeout: u32);
    pub fn nw_tcp_options_set_persist_timeout(options: nw_protocol_options_t,
                                              persist_timeout: u32);
    pub fn nw_tcp_options_set_retransmit_connection_drop_time(options: nw_protocol_options_t,
                                                              retransmit_connection_drop_time: u32);
    pub fn nw_tcp_options_set_retransmit_fin_drop(options: nw_protocol_options_t,
                                                  retransmit_fin_drop: bool);
    pub fn nw_tcp_options_set_disable_ack_stretching(options: nw_protocol_options_t,
                                                     disable_ack_stretching: bool);
    pub fn nw_tcp_options_set_enable_fast_open(options: nw_protocol_options_t,
                                               enable_fast_open: bool);
    pub fn nw_tcp_options_set_disable_ecn(options: nw_protocol_options_t,
                                          disable_ecn: bool);
    pub fn nw_protocol_metadata_is_tcp(metadata: nw_protocol_metadata_t) -> bool;
    pub fn nw_tcp_get_available_receive_buffer(metadata: nw_protocol_metadata_t) -> u32;
    pub fn nw_tcp_get_available_send_buffer(metadata: nw_protocol_metadata_t) -> u32;
}