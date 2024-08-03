//! The DJI RM-S1 DDS Protocol
//!
//! This module contains the definitions of the DDS protocol packets that are used in the RM-S1 protocol.

pub const CMDSET_DDS: u8 = 0x48;
pub const CMDID_DDS_ADD_SUB: u8 = 0x03;
pub const CMDID_DDS_DEL_SUB: u8 = 0x04;
pub const CMDID_DDS_RESET_NODE: u8 = 0x02;
pub const CMDID_DDS_PUSH_MSG: u8 = 0x08;

pub mod topic_view;
pub mod topics;

pub use topic_view::RMTopicView;
