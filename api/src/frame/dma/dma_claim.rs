use env::PieToken;

/// 关闭本次绑定：重复请求继续同一责任，Fault 可覆盖 Normal。
/// 认领一台：调用方本次 claim 的重试编号 ＋ 主人那一枚报活孔。
///
/// **不装设备号**：“哪一台”由“哪一枚设备门牌响了”回答（与普通设备那一面同一条口径）——
/// 门牌上挂着类联盟的 permit，准入就是既有 Operator/Identity 那一套。
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
#[frame(len = 32)]
pub struct DmaClaim {
    pub op: u8,
    pub id: u64,
    pub sensor: PieToken,
    pub back: PieToken,
}
