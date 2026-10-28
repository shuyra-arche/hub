use env::wire::Span as _;
use env::PieToken;
use wire::Message;

use super::vocab::CLAIM;

/// 认领：**要什么权**（种 / 取用 / 形态）＋ **主人那一枚**。**"哪一台"不在这帧里**——你 `find`
/// 的是哪一格，那一格上挂的就是哪一台那一份孔
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Claim {
    pub op: u8,
    pub kind: u8,
    pub access: u32,
    pub policy: u32,
    pub sensor: PieToken,
    pub back: PieToken,
}

impl Claim {
    /// 编一问（动作码固定 CLAIM）
    pub fn of(kind: u8, access: u32, policy: u32, sensor: PieToken, back: PieToken) -> Claim {
        Claim {
            op: CLAIM,
            kind,
            access,
            policy,
            sensor,
            back,
        }
    }
}

impl Message for Claim {
    type In = Claim;
    type Buf = [u8; Claim::LEN];
    const EMPTY: Self::Buf = [0u8; Claim::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    fn fetch(bytes: &[u8]) -> Option<Claim> {
        if bytes.len() != Claim::LEN {
            return None;
        }
        let q = Claim::fetch_at(bytes, 0)?.0;
        (q.op == CLAIM).then_some(q)
    }
}
