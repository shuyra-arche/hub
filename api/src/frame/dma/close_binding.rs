use env::PieToken;

/// 关闭本次绑定：重复请求继续同一责任，Fault 可覆盖 Normal。
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
#[frame(len = 32)]
pub struct CloseBinding {
    pub op: u8,
    pub serial: u64,
    pub reason: u8,
    pub back: PieToken,
}
