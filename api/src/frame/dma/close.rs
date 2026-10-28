/// 关闭理由：Fault 可覆盖 Normal，不反向恢复。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Close {
    Normal,
    Fault,
}

impl Close {
    pub const fn code(self) -> u8 {
        match self {
            Self::Normal => 1,
            Self::Fault => 2,
        }
    }

    pub const fn of_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Normal),
            2 => Some(Self::Fault),
            _ => None,
        }
    }
}
