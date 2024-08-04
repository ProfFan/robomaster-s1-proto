//! The DJI RM-S1 VBUS Protocol
//!
//! This module contains the definitions of the VBUS protocol packets that are used in the RM-S1 protocol.

pub const CMDSET_VBUS: u8 = 0x48;
pub const CMDID_VBUS_RESET_NODE: u8 = 0x02;
pub const CMDID_VBUS_ADD_SUB: u8 = 0x03;
pub const CMDID_VBUS_DEL_SUB: u8 = 0x04;
pub const CMDID_VBUS_PUSH_MSG: u8 = 0x08;

/// The VBUS protocol command set
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
