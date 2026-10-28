/// 失败记录：**实际失败方、操作与原失败码**。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Failure {
    pub party: u8,
    pub op: u8,
    pub code: u8,
}
