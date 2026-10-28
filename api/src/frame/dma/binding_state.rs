/// 绑定的协调状态（只属 Hub 的账，不复制 Driver 的寄存器读数）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindingState {
    /// 已建记录，发布前置还没备齐。
    Preparing,
    /// 前置全部满足，业务已放行。
    Active,
    /// 已拒绝新业务，停链与回收在推进。
    Closing,
    /// 本次责任全部结清，设备已可重新认领。
    Closed,
}

impl BindingState {
    pub const fn code(self) -> u8 {
        match self {
            Self::Preparing => 1,
            Self::Active => 2,
            Self::Closing => 3,
            Self::Closed => 4,
        }
    }

    pub const fn of_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Preparing),
            2 => Some(Self::Active),
            3 => Some(Self::Closing),
            4 => Some(Self::Closed),
            _ => None,
        }
    }
}
