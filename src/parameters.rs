use crate::*;

use libc::c_int;


pub enum nw_parameters {}
pub type nw_parameters_t = *mut nw_parameters;

pub enum nw_protocol_stack {}
pub type nw_protocol_stack_t = *mut nw_protocol_stack;

pub type nw_service_class_t = c_int;
pub const nw_service_class_best_effort: nw_service_class_t       = 0;
pub const nw_service_class_background: nw_service_class_t        = 1;
pub const nw_service_class_interactive_video: nw_service_class_t = 2;
pub const nw_service_class_interactive_voice: nw_service_class_t = 3;
pub const nw_service_class_responsive_data: nw_service_class_t   = 4;
pub const nw_service_class_signaling: nw_service_class_t         = 5;

pub type nw_multipath_service_t = c_int;
pub const nw_multipath_service_disabled: nw_multipath_service_t    = 0;
pub const nw_multipath_service_handover: nw_multipath_service_t    = 1;
pub const nw_multipath_service_interactive: nw_multipath_service_t = 2;
pub const nw_multipath_service_aggregate: nw_multipath_service_t   = 3;

pub type nw_parameters_expired_dns_behavior_t = c_int;
pub const nw_parameters_expired_dns_behavior_default: nw_parameters_expired_dns_behavior_t  = 0;
pub const nw_parameters_expired_dns_behavior_allow: nw_parameters_expired_dns_behavior_t    = 1;
pub const nw_parameters_expired_dns_behavior_prohibit: nw_parameters_expired_dns_behavior_t = 2;


extern "C" {
    // typedef void (^nw_parameters_configure_protocol_block_t)(nw_protocol_options_t options);
    // pub fn nw_parameters_create_secure_tcp(configure_tls: nw_parameters_configure_protocol_block_t,
    //                                        configure_tcp: nw_parameters_configure_protocol_block_t)
    //     -> nw_parameters_t;
    // pub fn nw_parameters_create_secure_udp(configure_dtls: nw_parameters_configure_protocol_block_t,
    //                                        configure_udp: nw_parameters_configure_protocol_block_t)
    //     -> nw_parameters_t;

    pub fn nw_parameters_create() -> nw_parameters_t;
    pub fn nw_parameters_copy(parameters: nw_parameters_t) -> nw_parameters_t;
    pub fn nw_parameters_require_interface(parameters: nw_parameters_t, interface: nw_interface_t);
    pub fn nw_parameters_copy_required_interface(parameters: nw_parameters_t) -> nw_interface_t;
    pub fn nw_parameters_prohibit_interface(parameters: nw_parameters_t, interface: nw_interface_t);
    pub fn nw_parameters_clear_prohibited_interfaces(parameters: nw_parameters_t);

    // typedef bool (^nw_parameters_iterate_interfaces_block_t)(nw_interface_t interface);
    // pub fn nw_parameters_iterate_prohibited_interfaces(parameters: nw_parameters_t,
    //                                                    iterate_block: nw_parameters_iterate_interfaces_block_t);

    pub fn nw_parameters_set_required_interface_type(parameters: nw_parameters_t) -> nw_interface_type_t;
    pub fn nw_parameters_prohibit_interface_type(parameters: nw_parameters_t, interface_type: nw_interface_type_t);
    pub fn nw_parameters_clear_prohibited_interface_types(parameters: nw_parameters_t);

    // typedef bool (^nw_parameters_iterate_interface_types_block_t)(nw_interface_type_t interface_type);
    // pub fn nw_parameters_iterate_prohibited_interface_types(parameters: nw_parameters_t,
    //                                                         iterate_block: nw_parameters_iterate_interface_types_block_t);

    pub fn nw_parameters_set_prohibit_expensive(parameters: nw_parameters_t, prohibit_expensive: bool);
    pub fn nw_parameters_get_prohibit_expensive(parameters: nw_parameters_t) -> bool;
    pub fn nw_parameters_set_reuse_local_address(parameters: nw_parameters_t, reuse_local_address: bool);
    pub fn nw_parameters_get_reuse_local_address(parameters: nw_parameters_t) -> bool;
    pub fn nw_parameters_set_local_endpoint(parameters: nw_parameters_t, local_endpoint: nw_endpoint_t);
    pub fn nw_parameters_copy_local_endpoint(parameters: nw_parameters_t) -> nw_endpoint_t;
    pub fn nw_parameters_set_include_peer_to_peer(parameters: nw_parameters_t, include_peer_to_peer: bool);
    pub fn nw_parameters_get_include_peer_to_peer(parameters: nw_parameters_t) -> bool;
    pub fn nw_parameters_set_fast_open_enabled(parameters: nw_parameters_t, fast_open_enabled: bool);
    pub fn nw_parameters_get_fast_open_enabled(parameters: nw_parameters_t) -> bool;

    pub fn nw_parameters_set_service_class(parameters: nw_parameters_t, service_class: nw_service_class_t);
    pub fn nw_parameters_get_service_class(parameters: nw_parameters_t) -> nw_service_class_t;

    pub fn nw_parameters_set_multipath_service(parameters: nw_parameters_t, multipath_service: nw_multipath_service_t);
    pub fn nw_parameters_get_multipath_service(parameters: nw_parameters_t) -> nw_multipath_service_t;

    pub fn nw_parameters_copy_default_protocol_stack(parameters: nw_parameters_t) -> nw_protocol_stack_t;
    pub fn nw_protocol_stack_prepend_application_protocol(stack: nw_protocol_stack_t, protocol: nw_protocol_options_t);

    pub fn nw_protocol_stack_clear_application_protocols(stack: nw_protocol_stack_t);

    // typedef void (^nw_protocol_stack_iterate_protocols_block_t)(nw_protocol_options_t protocol);
    // pub fn nw_protocol_stack_iterate_application_protocols(stack: nw_protocol_stack_t,
    //                                                        iterate_block: nw_protocol_stack_iterate_protocols_block_t);

    pub fn nw_protocol_stack_copy_transport_protocol(stack: nw_protocol_stack_t) -> nw_protocol_options_t;
    pub fn nw_protocol_stack_set_transport_protocol(stack: nw_protocol_stack_t, protocol: nw_protocol_options_t);
    pub fn nw_protocol_stack_copy_internet_protocol(stack: nw_protocol_stack_t) -> nw_protocol_options_t;
    pub fn nw_parameters_set_local_only(parameters: nw_parameters_t, local_only: bool);
    pub fn nw_parameters_get_local_only(parameters: nw_parameters_t) -> bool;
    pub fn nw_parameters_set_prefer_no_proxy(parameters: nw_parameters_t, prefer_no_proxy: bool);
    pub fn nw_parameters_get_prefer_no_proxy(parameters: nw_parameters_t) -> bool;
    pub fn nw_parameters_set_expired_dns_behavior(parameters: nw_parameters_t,
                                                 expired_dns_behavior: nw_parameters_expired_dns_behavior_t);
    pub fn nw_parameters_get_expired_dns_behavior(parameters: nw_parameters_t) -> nw_parameters_expired_dns_behavior_t;
    
}


