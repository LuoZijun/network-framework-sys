use crate::*;


pub enum nw_path_monitor {}
pub type nw_path_monitor_t = *mut nw_path_monitor;


extern "C" {
    pub fn nw_path_monitor_create() -> nw_path_monitor_t;
    pub fn nw_path_monitor_create_with_type(required_interface_type: nw_interface_type_t) -> nw_path_monitor_t;

    // typedef void (^nw_path_monitor_cancel_handler_t)(void);
    // pub fn nw_path_monitor_set_cancel_handler(monitor: nw_path_monitor_t,
    //                                           cancel_handler: nw_path_monitor_cancel_handler_t);

    // typedef void (^nw_path_monitor_update_handler_t) (nw_path_t path);
    // pub fn nw_path_monitor_set_update_handler(monitor: nw_path_monitor_t,
    //                                           update_handler: nw_path_monitor_update_handler_t);

    pub fn nw_path_monitor_set_queue(monitor: nw_path_monitor_t, queue: dispatch_queue_t);
    pub fn nw_path_monitor_start(monitor: nw_path_monitor_t);
    pub fn nw_path_monitor_cancel(monitor: nw_path_monitor_t);
    
}
