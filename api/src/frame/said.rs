use env::wire::Span as _;
use wire::Message;

/// 报名的答：只有状态那一格
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Said {
    pub status: u8,
}

impl Said {
    /// 编一答
    pub const fn of(status: u8) -> Said {
        Said { status }
    }
}

impl Message for Said {
    type In = Said;
    type Buf = [u8; Said::LEN];
    const EMPTY: Self::Buf = [0u8; Said::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    /// 恰好 Said::LEN（长短都不认）
    fn fetch(bytes: &[u8]) -> Option<Said> {
        if bytes.len() != Said::LEN {
            return None;
        }
        Said::fetch_at(bytes, 0).map(|one| one.0)
    }
}
