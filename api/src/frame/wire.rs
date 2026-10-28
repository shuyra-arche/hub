use alloc::string::String;
use env::PieToken;
use wire::Message;

use super::bond::Bond;
use super::claim::Claim;
use super::list::ListReq;
use super::vocab::{BOND, CLAIM, LIST};

/// **收进来的一问**（与 control 那一族的 `Wire` 同形）
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Wire {
    Bond(String),
    List(String, u32),
    Claim {
        kind: u8,
        access: u32,
        policy: u32,
        sensor: PieToken,
    },
}

impl Wire {
    /// 解一问：`(读出来的动作, 回信孔那一格)`
    /// **不认得的码 ⇒ `None`**（不回话）：三形长度不同，读不出"往哪回"那一格——不猜
    pub fn take(bytes: &[u8]) -> Option<(Option<Wire>, PieToken)> {
        match bytes.first().copied()? {
            BOND => {
                let q = Bond::fetch(bytes)?;
                Some((Some(Wire::Bond(q.class)), q.back))
            }
            LIST => {
                let q = ListReq::fetch(bytes)?;
                Some((Some(Wire::List(q.class, q.from)), q.back))
            }
            CLAIM => {
                let q = Claim::fetch(bytes)?;
                Some((
                    Some(Wire::Claim {
                        kind: q.kind,
                        access: q.access,
                        policy: q.policy,
                        sensor: q.sensor,
                    }),
                    q.back,
                ))
            }
            _ => None,
        }
    }
}
