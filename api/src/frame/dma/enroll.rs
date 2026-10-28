use alloc::string::String;
use env::PieToken;

/// Device registration metadata. Its source is the authenticated kernel sender.
#[derive(env::Frame, Clone, PartialEq, Eq, Debug)]
#[frame(len = 83)]
pub struct Enroll {
    pub op: u8,
    pub key: u64,
    pub line: u32,
    pub name: String,
    pub class: String,
    pub back: PieToken,
}
