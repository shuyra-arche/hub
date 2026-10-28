use alloc::string::String;
use env::wire::Span as _;
use env::PieToken;
use wire::Message;

use super::vocab::BOND;

/// 报名：**只有类**（驱动不需要知道盟号）
#[derive(env::Frame, Clone, PartialEq, Eq, Debug)]
#[frame(len = 41)]
pub struct Bond {
    pub op: u8,
    pub class: String,
    pub back: PieToken,
}

impl Bond {
    /// 编一问（动作码固定 BOND）
    pub fn of(class: String, back: PieToken) -> Bond {
        Bond {
            op: BOND,
            class,
            back,
        }
    }
}

impl Message for Bond {
    type In = Bond;
    type Buf = [u8; Bond::LEN];
    const EMPTY: Self::Buf = [0u8; Bond::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    /// **恰好**（按游标判：名字变长，帧长不再等于 Bond::LEN——那是上界）且动作码是 BOND
    fn fetch(bytes: &[u8]) -> Option<Bond> {
        let (q, at) = Bond::fetch_at(bytes, 0)?;
        (at == bytes.len() && q.op == BOND).then_some(q)
    }
}
