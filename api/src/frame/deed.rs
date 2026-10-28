use alloc::string::String;
use env::wire::Span as _;
use env::PieToken;
use wire::{Message, OK};

/// **契**：认领那一答。**是扁平的一形**（状态 ＋ 名字 ＋ 线号 ＋ 号）——本仓的答话都不嵌套
/// 另一枚帧（`Field` 只管一格多宽，不认复合），故"契"与"状态"同住这一枚
/// 三格各有各的消费者：`token` → Device::open（那一页在这一域表里是几号）；`line` →
#[derive(env::Frame, Clone, PartialEq, Eq, Debug)]
#[frame(len = 45)]
pub struct Deed {
    pub status: u8,
    pub name: String,
    pub line: u32,
    pub token: PieToken,
}

impl Deed {
    /// 空的一张：失败时那一答用它（`token` 是 PieToken::NONE）
    pub const NONE: Deed = Deed {
        status: 0,
        name: String::new(),
        line: 0,
        token: PieToken::NONE,
    };

    /// 编一答：只有状态那一格（失败，或读不懂）
    pub const fn of(status: u8) -> Deed {
        Deed {
            status,
            name: String::new(),
            line: 0,
            token: PieToken::NONE,
        }
    }

    /// 编一答：成了 ＋ 那张契
    pub const fn granted(name: String, line: u32, token: PieToken) -> Deed {
        Deed {
            status: OK,
            name,
            line,
            token,
        }
    }
}

impl Message for Deed {
    type In = Deed;
    type Buf = [u8; Deed::LEN];
    const EMPTY: Self::Buf = [0u8; Deed::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    /// **恰好**（按游标判：`name` 变长；长短都不认）
    fn fetch(bytes: &[u8]) -> Option<Deed> {
        let (q, at) = Deed::fetch_at(bytes, 0)?;
        (at == bytes.len()).then_some(q)
    }
}
