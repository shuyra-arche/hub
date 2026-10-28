use env::PieToken;

/// 整机退场门：关闭执行实例与开放绑定，报告未结清责任数。
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
#[frame(len = 16)]
pub struct Settle {
    pub op: u8,
    pub back: PieToken,
}
