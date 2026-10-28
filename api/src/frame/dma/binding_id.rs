use env::TaskId;

/// Hub 签发的绑定身份；序号不复用，序号耗尽终判 `Full`。
/// 未结清记录始终保留；最近 32 笔结清结果可查询，淘汰后查询返回 `Unknown`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BindingId {
    pub hub: TaskId,
    pub serial: u64,
}
