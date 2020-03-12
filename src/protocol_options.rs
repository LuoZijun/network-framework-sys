
pub enum nw_protocol_definition { }
pub type nw_protocol_definition_t = *mut nw_protocol_definition;

pub enum nw_protocol_options { }
pub type nw_protocol_options_t = *mut nw_protocol_options;

pub enum nw_protocol_metadata { }
pub type nw_protocol_metadata_t = *mut nw_protocol_metadata;


extern "C" {
    pub fn nw_protocol_definition_is_equal(definition1: nw_protocol_definition_t,
                                           definition2: nw_protocol_definition_t)
        -> bool;
    pub fn nw_protocol_options_copy_definition(options: nw_protocol_options_t) 
        -> nw_protocol_definition_t;
    pub fn nw_protocol_metadata_copy_definition(metadata: nw_protocol_metadata_t)
        -> nw_protocol_definition_t;

}