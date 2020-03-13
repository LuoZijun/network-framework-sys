use crate::*;

use libc::c_int;

pub type nw_ip_version_t = c_int;
pub const nw_ip_version_any: nw_ip_version_t = 0;
pub const nw_ip_version_4: nw_ip_version_t   = 4;
pub const nw_ip_version_6: nw_ip_version_t   = 6;

pub type nw_ip_ecn_flag_t = c_int;
pub const nw_ip_ecn_flag_non_ect: nw_ip_ecn_flag_t = 0;
pub const nw_ip_ecn_flag_ect_0: nw_ip_ecn_flag_t   = 2;
pub const nw_ip_ecn_flag_ect_1: nw_ip_ecn_flag_t   = 1;
pub const nw_ip_ecn_flag_ce: nw_ip_ecn_flag_t      = 3;


extern "C" {
    pub fn nw_protocol_copy_ip_definition() -> nw_protocol_definition_t;
    pub fn nw_ip_options_set_version(options: nw_protocol_options_t, version: nw_ip_version_t);
    pub fn nw_ip_options_set_hop_limit(options: nw_protocol_options_t, hop_limit: u8);
    pub fn nw_ip_options_set_use_minimum_mtu(options: nw_protocol_options_t, use_minimum_mtu: bool);
    pub fn nw_ip_options_set_disable_fragmentation(options: nw_protocol_options_t, disable_fragmentation: bool);
    pub fn nw_ip_options_set_calculate_receive_time(options: nw_protocol_options_t, calculate_receive_time: bool);
    
    pub fn nw_ip_create_metadata() -> nw_protocol_metadata_t;
    pub fn nw_protocol_metadata_is_ip(metadata: nw_protocol_metadata_t) -> bool;
    pub fn nw_ip_metadata_set_ecn_flag(metadata: nw_protocol_metadata_t, ecn_flag: nw_ip_ecn_flag_t);
    pub fn nw_ip_metadata_get_ecn_flag(metadata: nw_protocol_metadata_t) -> nw_ip_ecn_flag_t;
    pub fn nw_ip_metadata_set_service_class(metadata: nw_protocol_metadata_t, service_class: nw_service_class_t);
    pub fn nw_ip_metadata_get_service_class(metadata: nw_protocol_metadata_t) -> nw_service_class_t;
    pub fn nw_ip_metadata_get_receive_time(metadata: nw_protocol_metadata_t) -> u64;
}

