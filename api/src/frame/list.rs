use alloc::string::String;
use env::wire::Span as _;
use env::PieToken;
use wire::Message;

use super::vocab::LIST;

/// 列册：类 ＋ **游标**（从哪一条起取窗）
#[derive(env::Frame, Clone, PartialEq, Eq, Debug)]
#[frame(len = 45)]
pub struct ListReq {
    pub op: u8,
    pub class: String,
    pub from: u32,
    pub back: PieToken,
}

impl ListReq {
    /// 编一问（动作码固定 LIST）
    pub fn of(class: String, from: u32, back: PieToken) -> ListReq {
        ListReq {
            op: LIST,
            class,
            from,
            back,
        }
    }
}

impl Message for ListReq {
    type In = ListReq;
    type Buf = [u8; ListReq::LEN];
    const EMPTY: Self::Buf = [0u8; ListReq::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    /// **恰好**（按游标判：`class` 变长）
    fn fetch(bytes: &[u8]) -> Option<ListReq> {
        let (q, at) = ListReq::fetch_at(bytes, 0)?;
        (at == bytes.len() && q.op == LIST).then_some(q)
    }
}
