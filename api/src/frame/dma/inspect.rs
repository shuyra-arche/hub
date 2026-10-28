use env::PieToken;

/// 查看同一绑定的协调进度或原始失败。
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
#[frame(len = 32)]
pub struct Inspect {
    pub op: u8,
    pub serial: u64,
    pub back: PieToken,
}
