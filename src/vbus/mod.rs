//! The DJI RM-S1 DDS Protocol
//!
//! This module contains the definitions of the DDS protocol packets that are used in the RM-S1 protocol.

pub const CMDSET_DDS: u8 = 0x48;
pub const CMDID_DDS_RESET_NODE: u8 = 0x02;
pub const CMDID_DDS_ADD_SUB: u8 = 0x03;
pub const CMDID_DDS_DEL_SUB: u8 = 0x04;
pub const CMDID_DDS_PUSH_MSG: u8 = 0x08;

/// The DDS(VBUS) protocol command set
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VBusCmd {
    ADD_NODE = 0x01,
    NODE_RESET = 0x02,
    ADD_MSG = 0x03,
    DEL_MSG = 0x04,
    QUERY_CONF = 0x05,
    SET_PUSH_FREQ = 0x06,
    PUSH_CTRL = 0x07,
    DATA_ANALYSIS = 0x08,
}

pub mod topic_view;
pub mod topics;

pub use topic_view::RMTopicView;
